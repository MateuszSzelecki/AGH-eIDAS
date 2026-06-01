use crate::storage;
use crate::zkp_gen;
use ark_bn254::Fr;
use ark_crypto_primitives::sponge::CryptographicSponge;
use ark_crypto_primitives::sponge::poseidon::{find_poseidon_ark_and_mds, PoseidonConfig, PoseidonSponge};
use ark_ed_on_bn254::{EdwardsAffine as BJJAffine, Fr as BJJScalar};
use ark_ff::{UniformRand, PrimeField, BigInteger};
use ark_ec::{AffineRepr, CurveGroup};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use base64::{engine::general_purpose, Engine as _};
use log::info;
use rand::thread_rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

use uuid::Uuid;

// Local issuer secret key. Create wallet/src-tauri/assets/issuer_sk.bin locally and do not commit it.
const ISSUER_SK_BYTES: &[u8] = include_bytes!("../assets/issuer_sk.bin");

fn poseidon_config() -> PoseidonConfig<Fr> {
    let full_rounds = 8;
    let partial_rounds = 31;
    let alpha = 17;
    let rate = 2;
    let capacity = 1;

    let (ark, mds) = find_poseidon_ark_and_mds::<Fr>(
        Fr::MODULUS_BIT_SIZE as u64,
        rate,
        full_rounds as u64,
        partial_rounds as u64,
        0,
    );

    PoseidonConfig::new(full_rounds, partial_rounds, alpha, mds, ark, rate, capacity)
}

fn schnorr_sign(
    sk: &BJJScalar,
    msg: &Fr,
    generator: &BJJAffine,
    poseidon_cfg: &PoseidonConfig<Fr>,
    rng: &mut impl rand::Rng,
) -> (BJJAffine, Fr) {
    let k = BJJScalar::rand(rng);
    let r_point: BJJAffine = (generator.into_group() * k).into_affine();

    let mut sponge = PoseidonSponge::new(poseidon_cfg);
    sponge.absorb(&r_point.x);
    sponge.absorb(&r_point.y);
    sponge.absorb(msg);
    let e: Fr = sponge.squeeze_field_elements(1)[0];

    let e_scalar = BJJScalar::from_le_bytes_mod_order(&e.into_bigint().to_bytes_le());
    let s = k - e_scalar * sk;
    let s_fr = Fr::from_le_bytes_mod_order(&s.into_bigint().to_bytes_le());

    (r_point, s_fr)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChallengePayload {
    challenge_id: String,
    nonce: String,
    timestamp: u64,
    callback_url: String,
    verifier_name: String,
}

impl ChallengePayload {
    pub fn get_nonce(&self) -> u128 {
        let nonce_u128 = u128::from_str_radix(self.nonce.trim_start_matches("0x"), 16).unwrap();
        nonce_u128
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDocument {
    pub identifier: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: u64,
    pub issue_date: u64,
    pub expiry_date: u64,
    pub sig_r: String,
    pub sig_s: String,
}

#[derive(Serialize, Deserialize)]
struct PublicInputs {
    generation_date: u64,
    nonce: u128,
}

impl PublicInputs {
    fn new(generation_date: u64, nonce: u128) -> Self {
        Self {
            generation_date,
            nonce,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProofPayload {
    proof: String,
    public_inputs: PublicInputs,
}

impl ProofPayload {
    pub fn new(proof: String, generation_date: u64, nonce: u128) -> Self {
        Self {
            proof,
            public_inputs: PublicInputs::new(generation_date, nonce),
        }
    }
}

#[derive(Serialize)]
struct VerificationRequest {
    nonce: String,
    proof: Value,
}

// TO DO: create real implementation
// Remember to check authentication
#[tauri::command]
pub fn request_document(name: String, surname: String, date_of_birth: u64) -> UserDocument {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut rng = thread_rng();
    let poseidon_cfg = poseidon_config();
    let generator = BJJAffine::generator();

    let issuer_sk = BJJScalar::deserialize_compressed(ISSUER_SK_BYTES)
        .expect("Failed to load issuer SK");
    let birthdate_fr = Fr::from(date_of_birth);
    let (sig_r, sig_s) = schnorr_sign(&issuer_sk, &birthdate_fr, &generator, &poseidon_cfg, &mut rng);

    let mut r_bytes = Vec::new();
    let mut s_bytes = Vec::new();
    sig_r.serialize_compressed(&mut r_bytes).unwrap();
    sig_s.serialize_compressed(&mut s_bytes).unwrap();

    let document = UserDocument {
        identifier: Uuid::new_v4().to_string(),
        first_name: name,
        last_name: surname,
        date_of_birth,
        issue_date: now,
        expiry_date: now + 30 * 24 * 60 * 60, // One month from now
        sig_r: general_purpose::STANDARD.encode(r_bytes),
        sig_s: general_purpose::STANDARD.encode(s_bytes),
    };
    document
}

#[tauri::command]
pub fn load_document() -> UserDocument {
    let document: UserDocument = match storage::get_user_document() {
        Ok(s) => s,
        Err(_) => UserDocument {
            identifier: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            date_of_birth: 0,
            issue_date: 0,
            expiry_date: 0,
            sig_r: "".to_string(),
            sig_s: "".to_string(),
        },
    };

    document
}

#[tauri::command]
pub async fn generate_proof(challenge: ChallengePayload) -> Result<(), String> {
    info!("GENERATING PROOF");
    inner_generate_proof(challenge)
        .await
        .map_err(|e| e.to_string())
}

async fn inner_generate_proof(
    challenge: ChallengePayload,
) -> Result<(), Box<dyn std::error::Error>> {
    // TO DO: Verify Challenge
    // Right now it automatically sends proof
    let user_document = storage::get_user_document()?;

    info!("STARTING PROOF GENERATION");
    let proof = zkp_gen::generate_proof(user_document, challenge.clone())?;
    info!("PROOF GENERATED SENDING");
    send_proof(proof, challenge).await?;
    info!("PROOF SENT");
    Ok(())
}

async fn send_proof(
    proof: ProofPayload,
    challenge: ChallengePayload,
) -> Result<(), Box<dyn std::error::Error>> {
    // REMEMBER TO CHECK OF ON THE API SIDE IF NONCE FROM PUBLIC INPUTS IS CHECKED!!!!
    let client = Client::new();

    let res = client
        .post(&challenge.callback_url)
        .json(&proof)
        .send()
        .await?;

    info!("Status: {:?}", res);

    Ok(())
}
