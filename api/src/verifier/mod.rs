mod controller;
mod model;
mod service;

use std::{collections::HashMap, fmt::Display, sync::Mutex};

pub use controller::scope;

use crate::verifier::model::{Challenge, Nonce, SnarkJsVerificationKey};

#[derive(Debug, thiserror::Error)]
pub enum VerifierError {
    StoreChallengeFailed,
    ChallengeNotFound,
    InvalidVerificationKey,
    VerificationFailed,
}

impl actix_web::error::ResponseError for VerifierError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            VerifierError::StoreChallengeFailed | VerifierError::InvalidVerificationKey => {
                actix_web::HttpResponse::InternalServerError().body("Internal Server Error")
            }
            VerifierError::ChallengeNotFound => {
                actix_web::HttpResponse::NotFound().body("No challenge with the provided nonce")
            }
            VerifierError::VerificationFailed => {
                actix_web::HttpResponse::BadRequest().body("Verification failed")
            }
        }
    }
}

impl Display for VerifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifierError::StoreChallengeFailed => {
                writeln!(f, "Failed to store challenge!")
            }
            VerifierError::ChallengeNotFound => {
                writeln!(f, "Failed to get challenge for provided nonce!")
            }
            VerifierError::InvalidVerificationKey => {
                writeln!(f, "Invalid verification key!")
            }
            VerifierError::VerificationFailed => {
                writeln!(f, "Verification failed!")
            }
        }
    }
}

pub struct VerifierData {
    challenges: Mutex<HashMap<Nonce, Challenge>>,
    pub vk: SnarkJsVerificationKey,
}

impl VerifierData {
    fn new() -> Self {
        let vk_path = "../zk/artifacts/verification_key.json";
        let vk_str = std::fs::read_to_string(vk_path)
            .unwrap_or_else(|_| panic!("Failed to read verification key at {}", vk_path));
        let vk: SnarkJsVerificationKey =
            serde_json::from_str(&vk_str).expect("Failed to parse verification key");

        Self {
            challenges: Mutex::new(HashMap::new()),
            vk,
        }
    }

    pub fn store_challenge(&self, challenge: &Challenge) -> Result<(), VerifierError> {
        let mut challenges = self
            .challenges
            .lock()
            .map_err(|_| VerifierError::StoreChallengeFailed)?;
        challenges.insert(challenge.nonce.clone(), challenge.clone());
        Ok(())
    }

    pub fn get_challenge(&self, nonce: &Nonce) -> Option<Challenge> {
        let challenges = self.challenges.lock().ok()?;
        challenges.get(nonce).cloned()
    }
}
