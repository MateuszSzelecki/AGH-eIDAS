use actix_web::http::Uri;
use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, Proof as ArkProof, VerifyingKey, prepare_verifying_key};
use ark_serialize::CanonicalDeserialize;
use ark_snark::SNARK;
use base64::{Engine as _, engine::general_purpose};

use crate::verifier::{
    VerificationStatus, VerifierData, VerifierError,
    model::{Challenge, Nonce, PublicInputs},
};

pub fn handle_generate_challenge(verifier_data: &VerifierData) -> Result<Challenge, VerifierError> {
    log::info!("Requested a challenge");

    //Changed url for tests with android emulator
    let challenge = Challenge::new(
        Uri::from_static("http://10.0.2.2:3000/api/verifier/verify"),
        "TestVerifier",
    );

    log::info!("Generated challenge: {challenge:?}");

    verifier_data
        .challenges()
        .ok_or(VerifierError::StoreChallengeFailed)?
        .insert(challenge.nonce, challenge.clone());

    log::info!("Stored challenge for verification");

    Ok(challenge)
}

const VK_BYTES: &[u8] = include_bytes!("../../../zk/artifacts/vk.bin");
pub async fn handle_proof_verification(
    verifier_data: &VerifierData,
    proof: &str,
    proof_public_inputs: PublicInputs,
) -> Result<bool, VerifierError> {
    log::info!("Requested a verification for {proof:?}");

    let vk = VerifyingKey::<Bn254>::deserialize_compressed(VK_BYTES)
        .map_err(|_| VerifierError::VerificationFailed)?;
    let pvk = prepare_verifying_key(&vk);

    let public_inputs = vec![
        Fr::from(proof_public_inputs.generation_date),
        Fr::from(proof_public_inputs.nonce),
    ];

    let proof_bytes = general_purpose::STANDARD
        .decode(proof)
        .map_err(|_| VerifierError::VerificationFailed)?;

    let proof = ArkProof::<Bn254>::deserialize_compressed(&*proof_bytes)
        .map_err(|_| VerifierError::VerificationFailed)?;

    let verified = Groth16::<Bn254>::verify_with_processed_vk(&pvk, &public_inputs, &proof)
        .map_err(|_| VerifierError::VerificationFailed)?;

    log::info!("{:?}", verified);

    let nonce = Nonce::from_bytes(proof_public_inputs.nonce.to_be_bytes());

    let challenge = verifier_data
        .challenges()
        .and_then(|c| c.get(&nonce).cloned())
        .ok_or(VerifierError::ChallengeNotFound)?;

    if challenge.is_expired() {
        set_status(verifier_data, nonce, VerificationStatus::Expired).await?;
        return Ok(false);
    }

    let status = if verified {
        VerificationStatus::Success
    } else {
        VerificationStatus::Failure
    };
    set_status(verifier_data, nonce, status).await?;

    Ok(verified)
}

async fn set_status(
    verifier_data: &VerifierData,
    nonce: Nonce,
    status: VerificationStatus,
) -> Result<(), VerifierError> {
    log::info!("Challenge status for {nonce:?} changed to {status:?}");

    // If the challenge is not in the hashmap should we abort, or notify and then abort?
    verifier_data
        .challenges()
        .ok_or(VerifierError::ChallengeNotFound)?
        .remove(&nonce)
        .ok_or(VerifierError::ChallengeNotFound)?;

    let _ = verifier_data.status_tx.send((nonce, status)).await;
    Ok(())
}

pub async fn handle_verification_status(
    verifier_data: &VerifierData,
    nonce: Nonce,
) -> Result<VerificationStatus, VerifierError> {
    log::info!("Waiting for status update for {nonce:?}");

    let rx = verifier_data.status_rx.clone();
    loop {
        let (rx_nonce, status) = rx
            .recv()
            .await
            .map_err(|_| VerifierError::VerificationTimeout)?;
        if rx_nonce != nonce {
            continue;
        }

        log::info!("Recieved status update for {nonce:?}");
        return Ok(status);
    }
}

pub async fn expire_challenges(verifier_data: &VerifierData) {
    let nonces_to_cleanup = {
        let Some(challenges) = verifier_data.challenges() else {
            return;
        };
        challenges
            .iter()
            .filter(|(_, challenge)| challenge.is_expired())
            .map(|(nonce, _)| nonce.clone())
            .collect::<Vec<_>>()
    };
    for nonce in nonces_to_cleanup {
        log::info!("Rmoved expired challenge for {nonce:?}");
        let _ = set_status(verifier_data, nonce, VerificationStatus::Expired).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Captured from a real wallet-generated proof against the current zk/artifacts/vk.bin.
    // Regenerate via the wallet if circuit, proving key, or verifying key change.
    const VALID_PROOF_B64: &str = "AuWgWPDahC3f4nFnuynnzjjT5LEooqyiyHzcNr1BGi4qiMzL/mE7HzIYmpLPa3eZ5DfjH7c9hpQYBpisHvQuKCeTs0SC4APTMXyS7JDPL0VBg1DbAGqUOUrN37c7Z5KpP0eegU0lwqLzk1+BYqBu89CNQV3RX0XbhbCWnwo4lx4=";
    const VALID_GENERATION_DATE: u64 = 1778743280;
    const VALID_NONCE: u128 = 223782569634187167627945400628229207519;

    fn captured_inputs() -> PublicInputs {
        PublicInputs {
            generation_date: VALID_GENERATION_DATE,
            nonce: VALID_NONCE,
        }
    }

    #[actix_web::test]
    async fn verifies_captured_proof_against_stored_challenge() {
        let verifier_data = VerifierData::new();
        verifier_data.insert_test_challenge(VALID_NONCE);

        let verified = handle_proof_verification(&verifier_data, VALID_PROOF_B64, captured_inputs())
            .await
            .expect("verification call should succeed");

        assert!(verified, "captured proof should verify");
    }

    #[actix_web::test]
    async fn rejects_proof_without_matching_challenge() {
        let verifier_data = VerifierData::new();

        let result =
            handle_proof_verification(&verifier_data, VALID_PROOF_B64, captured_inputs()).await;

        assert!(
            matches!(result, Err(VerifierError::ChallengeNotFound)),
            "expected ChallengeNotFound, got {result:?}",
        );
    }

    #[actix_web::test]
    async fn rejects_proof_with_tampered_public_inputs() {
        let verifier_data = VerifierData::new();
        verifier_data.insert_test_challenge(VALID_NONCE);

        let mut inputs = captured_inputs();
        inputs.generation_date = inputs.generation_date.wrapping_add(1);

        let verified = handle_proof_verification(&verifier_data, VALID_PROOF_B64, inputs)
            .await
            .expect("verification call should succeed");

        assert!(!verified, "tampered public inputs must not verify");
    }

    #[actix_web::test]
    async fn rejects_garbage_proof_bytes() {
        let verifier_data = VerifierData::new();
        verifier_data.insert_test_challenge(VALID_NONCE);

        let result = handle_proof_verification(&verifier_data, "!!!not-base64!!!", captured_inputs())
            .await;

        assert!(
            matches!(result, Err(VerifierError::VerificationFailed)),
            "expected VerificationFailed for invalid proof bytes, got {result:?}",
        );
    }

    #[actix_web::test]
    async fn http_verify_endpoint_accepts_captured_payload() {
        use actix_web::{App, http::header, test, web};

        let verifier_data = web::Data::new(VerifierData::new());
        verifier_data.insert_test_challenge(VALID_NONCE);

        let app = test::init_service(
            App::new().service(crate::verifier::scope().app_data(verifier_data.clone())),
        )
        .await;

        // Sent as raw bytes because serde_json::Number cannot represent a u128 nonce
        // without the arbitrary_precision feature; this mirrors the literal payload
        // the wallet POSTs to /verifier/verify.
        let body = format!(
            r#"{{"proof":"{}","public_inputs":{{"generation_date":{},"nonce":{}}}}}"#,
            VALID_PROOF_B64, VALID_GENERATION_DATE, VALID_NONCE,
        );

        let req = test::TestRequest::post()
            .uri("/verifier/verify")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "expected 2xx for valid captured proof, got {}",
            resp.status()
        );
    }
}
