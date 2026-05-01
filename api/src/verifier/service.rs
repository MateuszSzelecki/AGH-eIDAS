use actix_web::http::Uri;

use crate::verifier::{
    VerifierData, VerifierError,
    model::{Challenge, Nonce, Proof},
};

pub fn generate_challenge() -> Challenge {
    let callback_url = Uri::from_static("https://127.0.0.1:3000/api/verifier/verify");
    Challenge::new(callback_url, "TestVerifier")
}

pub fn verify_proof(
    verifier_data: &VerifierData,
    nonce: &Nonce,
    proof: &Proof,
) -> Result<bool, VerifierError> {
    let challenge = verifier_data
        .get_challenge(nonce)
        .ok_or(VerifierError::ChallengeNotFound)?;

    log::info!("Found {challenge:?}");

    Ok(false)
}
