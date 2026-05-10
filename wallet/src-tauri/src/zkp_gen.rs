use crate::zkp::{ChallengePayload, ProofPayload, UserDocument};

use base64::{engine::general_purpose, Engine as _};

use log::info;

use ark_bn254::{Bn254, Fr};
use ark_ff::PrimeField;

use rand::thread_rng;

use std::time::{SystemTime, UNIX_EPOCH};

use ark_ec::AffineRepr;

use ark_r1cs_std::convert::ToBitsGadget;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::groups::curves::twisted_edwards::AffineVar;
use ark_r1cs_std::groups::CurveVar;
use ark_r1cs_std::{alloc::AllocVar, boolean::Boolean, fields::fp::FpVar};

use ark_groth16::Groth16;
use ark_groth16::ProvingKey;
use ark_relations::r1cs::*;
use ark_snark::SNARK;

use ark_std::vec::Vec;

use ark_ed_on_bn254::{EdwardsAffine as BJJAffine, EdwardsConfig};

use ark_crypto_primitives::sponge::constraints::CryptographicSpongeVar;
use ark_crypto_primitives::sponge::poseidon::constraints::PoseidonSpongeVar;
use ark_crypto_primitives::sponge::poseidon::{find_poseidon_ark_and_mds, PoseidonConfig};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

const EIGHTEEN_YEARS_IN_SECONDS: u64 = 567648000;

const PK_BYTES: &[u8] = include_bytes!("../assets/pk.bin");

const ISSUER_PK_BYTES: &[u8] = include_bytes!("../assets/issuer_pk.bin");

type EdwardsVar = AffineVar<EdwardsConfig, FpVar<Fr>>;

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

/// The circuit definition
#[derive(Clone)]
pub struct AgeVerifier {
    // Private inputs
    pub birthdate: Option<Fr>,
    pub issue_date: Option<Fr>,
    pub expiry_date: Option<Fr>,

    // Schnorr signature components (private)
    pub sig_r: Option<BJJAffine>,
    pub sig_s: Option<Fr>,

    // Public input (will be exposed)
    pub generation_date: Option<Fr>,
    pub nonce: Option<Fr>,

    // Circuit config (not witness, baked into circuit structure)
    pub poseidon_config: PoseidonConfig<Fr>,
    pub issuer_pk: BJJAffine,
    pub generator: BJJAffine,
}

impl ConstraintSynthesizer<Fr> for AgeVerifier {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<()> {
        // Witness and input setup
        let birthdate = FpVar::<Fr>::new_witness(cs.clone(), || {
            self.birthdate.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let issue_date = FpVar::<Fr>::new_witness(cs.clone(), || {
            self.issue_date.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let expiry_date = FpVar::<Fr>::new_witness(cs.clone(), || {
            self.expiry_date.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let sig_r_var = EdwardsVar::new_witness(cs.clone(), || {
            self.sig_r.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let sig_s_var = FpVar::<Fr>::new_witness(cs.clone(), || {
            self.sig_s.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let generation_date = FpVar::<Fr>::new_input(cs.clone(), || {
            self.generation_date
                .ok_or(SynthesisError::AssignmentMissing)
        })?;

        let nonce = FpVar::<Fr>::new_input(cs.clone(), || {
            self.nonce.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let generator_var = EdwardsVar::new_constant(cs.clone(), self.generator)?;
        let issuer_pk_var = EdwardsVar::new_constant(cs.clone(), self.issuer_pk)?;
        let eighteen_years = FpVar::<Fr>::Constant(Fr::from(EIGHTEEN_YEARS_IN_SECONDS));

        // calculate cutoff for proof generation
        let cutoff = generation_date.clone() - eighteen_years;

        // check if 18 yo, and if document is valid at the time of generation
        birthdate.enforce_cmp(&cutoff, std::cmp::Ordering::Less, true)?;
        issue_date.enforce_cmp(&generation_date, std::cmp::Ordering::Less, true)?;
        generation_date.enforce_cmp(&expiry_date, std::cmp::Ordering::Less, true)?;

        // Check document signature from shnoor and poseidon hash
        let mut sponge_var = PoseidonSpongeVar::new(cs.clone(), &self.poseidon_config);
        sponge_var.absorb(&sig_r_var.x)?;
        sponge_var.absorb(&sig_r_var.y)?;
        sponge_var.absorb(&birthdate)?;
        let e_var: Vec<FpVar<Fr>> = sponge_var.squeeze_field_elements(1)?;
        let e_var = &e_var[0];

        let s_bits: Vec<Boolean<Fr>> = sig_s_var.to_bits_le()?;
        let e_bits: Vec<Boolean<Fr>> = e_var.to_bits_le()?;

        let s_times_g = generator_var.scalar_mul_le(s_bits.iter())?;
        let e_times_pk = issuer_pk_var.scalar_mul_le(e_bits.iter())?;
        let computed_r = s_times_g + e_times_pk;
        computed_r.enforce_equal(&sig_r_var)?;

        let _inputs: Vec<FpVar<Fr>> = vec![
            birthdate,
            issue_date,
            expiry_date,
            generation_date.clone(),
            nonce.clone(),
        ];

        Ok(())
    }
}

pub fn generate_proof(
    user_data: UserDocument,
    challenge: ChallengePayload,
) -> core::result::Result<ProofPayload, Box<dyn std::error::Error>> {
    info!("GETTING DATA READY");
    // Settning up data for circuit
    let mut rng = thread_rng();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let birthdate_fr = Fr::from(user_data.date_of_birth);
    let issue_date = Fr::from(user_data.issue_date);
    let expiry_date = Fr::from(user_data.expiry_date);

    // Decode and deserialize shnoor signature from document
    let r_bytes = general_purpose::STANDARD.decode(user_data.sig_r)?;
    let s_bytes = general_purpose::STANDARD.decode(user_data.sig_s)?;
    let sig_r = BJJAffine::deserialize_compressed(&*r_bytes)?;
    let sig_s = Fr::deserialize_compressed(&*s_bytes)?;

    info!("{:?} {:?}", sig_r, sig_s);
    // Set up for poseidon hash generator
    let poseidon_cfg = poseidon_config();
    let generator = BJJAffine::generator();

    // Load shnoor public key from bytes
    let issuer_pk = BJJAffine::deserialize_compressed(ISSUER_PK_BYTES)?;

    // circuit set up
    let circuit = AgeVerifier {
        birthdate: Some(birthdate_fr),
        issue_date: Some(issue_date),
        expiry_date: Some(expiry_date),
        sig_r: Some(sig_r),
        sig_s: Some(sig_s),
        generation_date: Some(Fr::from(now)),
        nonce: Some(Fr::from(challenge.get_nonce())),
        poseidon_config: poseidon_cfg.clone(),
        issuer_pk,
        generator,
    };
    info!("LOADING PROVING KEY");
    // load circuit proving key from bytes
    let pk = ProvingKey::<Bn254>::deserialize_compressed(PK_BYTES)?;

    info!("PROVING");
    // generate proof
    let proof = Groth16::<Bn254>::prove(&pk, circuit.clone(), &mut rng)?;
    info!("{:?}", proof);
    // !!! IMPORTANT right now if the proof cannot exist the generation process will hang here
    info!("PROVED");

    // Serialize proof, encode into base64 and return for further usage
    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes).unwrap();

    Ok(ProofPayload::new(
        general_purpose::STANDARD.encode(proof_bytes),
        now,
        challenge.get_nonce(),
    ))
}
