use ark_bn254::Fr;
use ark_crypto_primitives::sponge::CryptographicSponge;
use ark_crypto_primitives::sponge::poseidon::{find_poseidon_ark_and_mds, PoseidonConfig, PoseidonSponge};
use ark_ed_on_bn254::{EdwardsAffine as BJJAffine, Fr as BJJScalar};
use ark_ff::{UniformRand, PrimeField, BigInteger};
use ark_ec::{AffineRepr, CurveGroup};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use base64::{engine::general_purpose, Engine as _};

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

// Signs a birthdate using the issuer's private key and returns (sig_r, sig_s) in base64 format.
pub fn sign_birthdate(
    sk_bytes: &[u8],
    date_of_birth: u64,
) -> Result<(String, String), String> {
    let mut rng = rand::thread_rng();
    let poseidon_cfg = poseidon_config();
    let generator = BJJAffine::generator();

    let issuer_sk = BJJScalar::deserialize_compressed(sk_bytes)
        .map_err(|e| format!("Failed to deserialize issuer private key: {}", e))?;
    
    let birthdate_fr = Fr::from(date_of_birth);
    let (sig_r, sig_s) = schnorr_sign(&issuer_sk, &birthdate_fr, &generator, &poseidon_cfg, &mut rng);

    let mut r_bytes = Vec::new();
    let mut s_bytes = Vec::new();
    sig_r.serialize_compressed(&mut r_bytes).map_err(|e| e.to_string())?;
    sig_s.serialize_compressed(&mut s_bytes).map_err(|e| e.to_string())?;

    let sig_r_b64 = general_purpose::STANDARD.encode(r_bytes);
    let sig_s_b64 = general_purpose::STANDARD.encode(s_bytes);

    Ok((sig_r_b64, sig_s_b64))
}
