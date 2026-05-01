use actix_web::http::Uri;

use crate::verifier::{
    VerifierData, VerifierError,
    model::{Challenge, Nonce, Proof},
};

pub fn handle_generate_challenge(verifier_data: &VerifierData) -> Result<Challenge, VerifierError> {
    log::info!("Requested a challenge");

    let challenge = Challenge::new(
        Uri::from_static("https://127.0.0.1:3000/api/verifier/verify"),
        "TestVerifier",
    );

    log::info!("Generated challenge: {challenge:?}");

    verifier_data.store_challenge(&challenge)?;

    log::info!("Stored challenge for verification");

    Ok(challenge)
}

pub fn handle_proof_verification(
    verifier_data: &VerifierData,
    nonce: &Nonce,
    proof: &Proof,
) -> Result<bool, VerifierError> {
    log::info!("Requested a verification for {nonce:?} and {proof:?}");

    let challenge = verifier_data
        .get_challenge(nonce)
        .ok_or(VerifierError::ChallengeNotFound)?;

    log::info!("Found {challenge:?}");

    let is_valid = false;

    if is_valid {
        log::info!("Verification successful");
        Ok(true)
    } else {
        log::info!("Verification failed");
        Ok(false)
    }
}
