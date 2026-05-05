use actix_web::http::Uri;
use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G2Affine};
use ark_groth16::{Groth16, Proof as ArkProof, VerifyingKey};
use ark_snark::SNARK;
use num_bigint::BigUint;
use std::str::FromStr;

use crate::verifier::{
    VerificationStatus, VerifierData, VerifierError,
    model::{Challenge, Nonce, Proof, SnarkJsVerificationKey},
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

fn parse_g1(coords: &[String; 3]) -> Result<G1Affine, VerifierError> {
    let x = Fq::from(BigUint::from_str(&coords[0]).map_err(|_| VerifierError::VerificationFailed)?);
    let y = Fq::from(BigUint::from_str(&coords[1]).map_err(|_| VerifierError::VerificationFailed)?);
    Ok(G1Affine::new_unchecked(x, y))
}

fn parse_g2(coords: &[[String; 2]; 3]) -> Result<G2Affine, VerifierError> {
    let x_re =
        Fq::from(BigUint::from_str(&coords[0][0]).map_err(|_| VerifierError::VerificationFailed)?);
    let x_im =
        Fq::from(BigUint::from_str(&coords[0][1]).map_err(|_| VerifierError::VerificationFailed)?);
    let y_re =
        Fq::from(BigUint::from_str(&coords[1][0]).map_err(|_| VerifierError::VerificationFailed)?);
    let y_im =
        Fq::from(BigUint::from_str(&coords[1][1]).map_err(|_| VerifierError::VerificationFailed)?);

    let x = Fq2::new(x_re, x_im);
    let y = Fq2::new(y_re, y_im);
    Ok(G2Affine::new_unchecked(x, y))
}

fn convert_vk(vk: &SnarkJsVerificationKey) -> Result<VerifyingKey<Bn254>, VerifierError> {
    let alpha_g1 = parse_g1(&vk.vk_alpha_1)?;
    let beta_g2 = parse_g2(&vk.vk_beta_2)?;
    let gamma_g2 = parse_g2(&vk.vk_gamma_2)?;
    let delta_g2 = parse_g2(&vk.vk_delta_2)?;

    let mut ic = Vec::new();
    for point in &vk.ic {
        ic.push(parse_g1(point)?);
    }

    Ok(VerifyingKey {
        alpha_g1,
        beta_g2,
        gamma_g2,
        delta_g2,
        gamma_abc_g1: ic,
    })
}

fn convert_proof(proof: &Proof) -> Result<ArkProof<Bn254>, VerifierError> {
    let a = parse_g1(&proof.pi_a)?;
    let b = parse_g2(&proof.pi_b)?;
    let c = parse_g1(&proof.pi_c)?;

    Ok(ArkProof { a, b, c })
}

pub async fn handle_proof_verification(
    verifier_data: &VerifierData,
    nonce: Nonce,
    proof: &Proof,
) -> Result<bool, VerifierError> {
    log::info!("Requested a verification for {nonce:?} and {proof:?}");

    let challenge = verifier_data
        .challenges()
        .and_then(|c| c.get(&nonce).cloned())
        .ok_or(VerifierError::ChallengeNotFound)?;

    if challenge.is_expired() {
        set_status(verifier_data, nonce, VerificationStatus::Expired).await?;
        return Ok(false);
    }

    // 1. Convert VK and Proof to Arkworks types
    let vk = convert_vk(&verifier_data.vk)?;
    let ark_proof = convert_proof(proof)?;

    // 2. Convert public signals to Fr
    let mut public_inputs = Vec::new();
    for signal in &proof.public_signals {
        let fr =
            Fr::from(BigUint::from_str(signal).map_err(|_| VerifierError::VerificationFailed)?);
        public_inputs.push(fr);
    }

    // 3. Verify the proof
    let pvk = Groth16::<Bn254>::process_vk(&vk).map_err(|_| VerifierError::VerificationFailed)?;
    let is_valid = Groth16::<Bn254>::verify_proof(&pvk, &ark_proof, &public_inputs)
        .map_err(|_| VerifierError::VerificationFailed)?;

    if !is_valid {
        log::info!("Groth16 verification failed");
        return Ok(false);
    }

    // 4. Check business logic: isValid (first public signal) must be 1
    if public_inputs.is_empty() || public_inputs[0] != Fr::from(1u32) {
        log::info!("Circuit logic failed: isValid is not 1");
        set_status(verifier_data, nonce, VerificationStatus::Failure).await?;
        return Ok(false);
    }

    // TODO: Verify nonce binding (Poseidon(nonce) == public_inputs[1])
    // For now we assume the nonce is correct if the proof is valid,
    // but in production we MUST check the echo.

    log::info!("Verification successful");
    set_status(verifier_data, nonce, VerificationStatus::Success).await?;
    Ok(true)
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
