use crate::zkp::{ChallengePayload, UserDocument};

use ark_bn254::{Bn254, Fr};
use ark_ff::PrimeField;

use rand::thread_rng;

use ark_ec::pairing::Pairing;
use ark_groth16::Proof;
use serde::Serialize;
use serde_json::to_writer_pretty;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs::File, path::Path};

use ark_ec::AffineRepr;
use ark_ff::BigInteger;
use num_bigint::BigUint;

use ark_r1cs_std::{alloc::AllocVar, fields::fp::FpVar};

use ark_groth16::VerifyingKey;

use ark_groth16::Groth16;
use ark_relations::r1cs::*;
use ark_snark::SNARK;

use ark_std::vec::Vec;

const EIGHTEEN_YEARS_IN_SECONDS: u64 = 567648000;

/// The circuit definition
#[derive(Clone)]
pub struct AgeVerifier {
    // Private inputs
    pub birthdate: Option<Fr>,
    pub signature: Option<Fr>,

    // Public input (will be exposed)
    pub generation_date: Option<Fr>,
    pub nonce: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for AgeVerifier {
    //TO DO: Implement real age verifier
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<()> {
        let birthdate = FpVar::<Fr>::new_witness(cs.clone(), || {
            self.birthdate.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let _signature = FpVar::<Fr>::new_witness(cs.clone(), || {
            self.signature.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let generation_date = FpVar::<Fr>::new_input(cs.clone(), || {
            self.generation_date
                .ok_or(SynthesisError::AssignmentMissing)
        })?;

        let nonce = FpVar::<Fr>::new_input(cs.clone(), || {
            self.nonce.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let eighteen_years = FpVar::<Fr>::Constant(Fr::from(EIGHTEEN_YEARS_IN_SECONDS));

        let cutoff = generation_date.clone() - eighteen_years;
        birthdate.enforce_cmp(&cutoff, std::cmp::Ordering::Less, true)?;

        let _inputs: Vec<FpVar<Fr>> = vec![birthdate, generation_date.clone(), nonce.clone()];

        Ok(())
    }
}

pub fn generate_proof(user_data: UserDocument, _challenge: ChallengePayload) -> Result<String> {
    // TO DO: Use real data
    // And remove underscores if used
    let mut rng = thread_rng();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let circuit = AgeVerifier {
        birthdate: Some(Fr::from(user_data.date_of_birth)),
        nonce: Some(Fr::from(1u128)),
        signature: Some(Fr::from(1234u128)),
        generation_date: Some(Fr::from(now)),
    };

    let (pk, _vk) = Groth16::<Bn254>::circuit_specific_setup(circuit.clone(), &mut rng)
        .expect("Failed to generate parameters");

    let proof =
        Groth16::<Bn254>::prove(&pk, circuit.clone(), &mut rng).expect("Failed to generate proof");

    let public_inputs = vec![circuit.generation_date.unwrap(), circuit.nonce.unwrap()];

    let prf = serde_json::to_string(
        &export_proof::<Bn254, _>(&proof, &public_inputs, "proof.json")
            .expect("Failed to export proof"),
    )
    .expect("proof generation failed");

    Ok(prf)
}

// From ark_snarkjs, i don't remember why it is copied and if i changed anything, but probably now it can be using crate now
#[derive(Debug)]
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct ProofJson {
    pub protocol: &'static str,     // always "groth16"
    pub curve: &'static str,        // "bn128" or "bls12381"
    pub pi_a: [String; 3],          // G1 point [x, y, 1]
    pub pi_b: [[String; 2]; 3],     // G2 point [[x0, x1], [y0, y1], [1, 0]]
    pub pi_c: [String; 3],          // G1 point [x, y, 1]
    pub publicSignals: Vec<String>, // array of decimal-encoded public inputs
}

/// Extract proof coordinates and convert public signals to decimal strings.
fn extract_proof_data<E>(
    proof: &Proof<E>,
    public: &[E::ScalarField],
) -> ([String; 2], [[String; 2]; 2], [String; 2], Vec<String>)
where
    E: Pairing + CurveTag,
    <E::G1Affine as ark_ec::AffineRepr>::BaseField: PrimeField,
    <E::G2Affine as ark_ec::AffineRepr>::BaseField: AsFp2,
    E::ScalarField: PrimeField,
{
    let a = g1_xy(&proof.a);
    let b = g2_xyxy(&proof.b);
    let c = g1_xy(&proof.c);
    let public_signals = public.iter().map(f_to_dec::<E::ScalarField>).collect();
    (a, b, c, public_signals)
}

/// Write a serializable JSON structure to a file, creating parent directories if needed.
//fn write_json_file<T, P>(json: &T, out_path: P) -> std::io::Result<()>
//where
//    T: Serialize,
//    P: AsRef<Path>,
//{
// Write pretty-printed JSON to file
//    let file = File::create(out_path)?;
//    to_writer_pretty(file, json).map_err(std::io::Error::other)?;
//    Ok(())
//}

/// Export a Groth16 proof and its public signals to `snarkjs` JSON format.
/// Writes the file to `out_path` and returns the in-memory `ProofJson`.
pub fn export_proof<E, P>(
    proof: &Proof<E>,          // Groth16 proof from arkworks
    public: &[E::ScalarField], // list of public inputs
    _out_path: P,              // output path for JSON file
) -> std::io::Result<ProofJson>
where
    P: AsRef<Path>,        // accepts &str, String, Path, PathBuf
    E: Pairing + CurveTag, // curve type with snarkjs "NAME"
    <E::G1Affine as ark_ec::AffineRepr>::BaseField: PrimeField,
    <E::G2Affine as ark_ec::AffineRepr>::BaseField: AsFp2,
    E::ScalarField: PrimeField,
{
    let (a, b, c, public_signals) = extract_proof_data(proof, public);

    let json = ProofJson {
        protocol: "groth16",
        curve: E::NAME,
        pi_a: [a[0].clone(), a[1].clone(), "1".to_string()],
        pi_b: [
            [b[0][0].clone(), b[0][1].clone()],
            [b[1][0].clone(), b[1][1].clone()],
            ["1".to_string(), "0".to_string()],
        ],
        pi_c: [c[0].clone(), c[1].clone(), "1".to_string()],
        publicSignals: public_signals,
    };

    //  write_json_file(&json, &out_path)?;
    Ok(json)
}

#[derive(Debug)]
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct VkJson {
    pub protocol: &'static str,
    pub curve: &'static str,
    pub nPublic: usize,

    #[serde(rename = "vk_alpha_1")]
    pub vk_alpha_1: [String; 3],

    #[serde(rename = "vk_beta_2")]
    pub vk_beta_2: [[String; 2]; 3],
    #[serde(rename = "vk_gamma_2")]
    pub vk_gamma_2: [[String; 2]; 3],
    #[serde(rename = "vk_delta_2")]
    pub vk_delta_2: [[String; 2]; 3],

    #[serde(rename = "IC")]
    pub ic: Vec<[String; 3]>,
}

pub fn vk_to_snarkjs<E>(vk: &VerifyingKey<E>, n_public: usize) -> VkJson
where
    E: Pairing + CurveTag,
    <E::G1Affine as ark_ec::AffineRepr>::BaseField: PrimeField,
    <E::G2Affine as ark_ec::AffineRepr>::BaseField: AsFp2,
{
    VkJson {
        protocol: "groth16",
        curve: E::NAME,
        nPublic: n_public,

        vk_alpha_1: g1_xyz(&vk.alpha_g1),
        vk_beta_2: g2_xyxy_z(&vk.beta_g2),
        vk_gamma_2: g2_xyxy_z(&vk.gamma_g2),
        vk_delta_2: g2_xyxy_z(&vk.delta_g2),

        ic: vk.gamma_abc_g1.iter().map(g1_xyz).collect(),
    }
}

pub fn export_vk<E, P>(
    vk: &VerifyingKey<E>,
    n_public: usize,
    out_path: P,
) -> std::io::Result<VkJson>
where
    P: AsRef<Path>,
    E: Pairing + CurveTag,
    <E::G1Affine as ark_ec::AffineRepr>::BaseField: PrimeField,
    <E::G2Affine as ark_ec::AffineRepr>::BaseField: AsFp2,
{
    let json = vk_to_snarkjs::<E>(vk, n_public);

    let file = File::create(out_path)?;
    to_writer_pretty(file, &json).map_err(std::io::Error::other)?;

    Ok(json)
}

pub trait CurveTag {
    const NAME: &'static str;
}

impl CurveTag for ark_bn254::Bn254 {
    const NAME: &'static str = "bn128";
}

/// Trait to access c0/c1 components of quadratic extension fields (Fp2).
pub trait AsFp2 {
    type Base: PrimeField;
    fn c0_c1(&self) -> (&Self::Base, &Self::Base);
}

impl<P> AsFp2 for ark_ff::fields::models::QuadExtField<P>
where
    P: ark_ff::fields::models::quadratic_extension::QuadExtConfig,
    P::BaseField: PrimeField,
{
    type Base = P::BaseField;
    fn c0_c1(&self) -> (&Self::Base, &Self::Base) {
        (&self.c0, &self.c1)
    }
}

/// Convert a field element to decimal string (snarkjs expects decimal format).
pub fn f_to_dec<F: PrimeField>(f: &F) -> String {
    let bi = f.into_bigint();
    BigUint::from_bytes_be(&bi.to_bytes_be()).to_str_radix(10)
}

/// Convert a G1 point to string array [x, y].
pub fn g1_xy<G>(p: &G) -> [String; 2]
where
    G: AffineRepr,
    G::BaseField: PrimeField,
{
    let (x, y) = p.xy().expect("G1 point at infinity?");
    [f_to_dec(&x), f_to_dec(&y)]
}

/// Convert a G2 point to nested string array [[x.c0, x.c1], [y.c0, y.c1]].
pub fn g2_xyxy<G>(p: &G) -> [[String; 2]; 2]
where
    G: AffineRepr,
    G::BaseField: AsFp2,
{
    let (x, y) = p.xy().expect("G2 point at infinity?");
    let (x0, x1) = x.c0_c1();
    let (y0, y1) = y.c0_c1();
    [[f_to_dec(x0), f_to_dec(x1)], [f_to_dec(y0), f_to_dec(y1)]]
}

/// Convert a G1 point to string array [x, y, z] with z=1 (normalized projective).
pub fn g1_xyz<G>(p: &G) -> [String; 3]
where
    G: AffineRepr,
    G::BaseField: PrimeField,
{
    let [x, y] = g1_xy(p);
    [x, y, "1".to_string()]
}

/// Convert a G2 point to [[[x0,x1],[y0,y1],[z0,z1]]] with z=(1,0) (normalized projective).
pub fn g2_xyxy_z<G>(p: &G) -> [[String; 2]; 3]
where
    G: AffineRepr,
    G::BaseField: AsFp2,
{
    let [x, y] = g2_xyxy(p);
    [x, y, ["1".to_string(), "0".to_string()]]
}
