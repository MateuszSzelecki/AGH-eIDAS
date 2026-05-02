mod controller;
mod model;
mod service;

use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex},
};

pub use controller::scope;

use crate::verifier::model::{Challenge, Nonce, SnarkJsVerificationKey};

#[derive(Debug, thiserror::Error)]
pub enum VerifierError {
    StoreChallengeFailed,
    ChallengeNotFound,
    InvalidVerificationKey,
    VerificationFailed,
    VerificationTimeout,
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
            VerifierError::VerificationTimeout => {
                actix_web::HttpResponse::RequestTimeout().body("Verification timed out")
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
            VerifierError::VerificationTimeout => {
                writeln!(f, "Verification timed out!")
            }
        }
    }
}

enum VerificationStatus {
    Success,
    Failure,
}
pub struct VerifierData {
    challenges: Arc<Mutex<HashMap<Nonce, Challenge>>>,
    status_tx: kanal::AsyncSender<(Nonce, VerificationStatus)>,
    status_rx: kanal::AsyncReceiver<(Nonce, VerificationStatus)>,
    pub vk: SnarkJsVerificationKey,
}
impl VerifierData {
    pub fn new() -> Self {
        let vk_path = "../zk/artifacts/verification_key.json";
        let vk_str = std::fs::read_to_string(vk_path)
            .unwrap_or_else(|_| panic!("Failed to read verification key at {}", vk_path));
        let vk: SnarkJsVerificationKey =
            serde_json::from_str(&vk_str).expect("Failed to parse verification key");

        let status_channel = kanal::unbounded_async();
        Self {
            challenges: Arc::new(Mutex::new(HashMap::new())),
            status_tx: status_channel.0,
            status_rx: status_channel.1,
            vk,
        }
    }

    fn store_challenge(&self, challenge: &Challenge) -> Result<(), VerifierError> {
        let mut challenges = self
            .challenges
            .lock()
            .map_err(|_| VerifierError::StoreChallengeFailed)?;
        challenges.insert(challenge.nonce, challenge.clone());
        Ok(())
    }

    fn get_challenge(&self, nonce: Nonce) -> Option<Challenge> {
        let challenges = self.challenges.lock().ok()?;
        challenges.get(&nonce).cloned()
    }

    async fn set_status(
        &self,
        nonce: Nonce,
        status: VerificationStatus,
    ) -> Result<(), VerifierError> {
        // If the challenge is not in the hashmap should we abort, or notify and then abort?
        {
            let mut challenges = self
                .challenges
                .lock()
                .map_err(|_| VerifierError::ChallengeNotFound)?;

            challenges
                .remove(&nonce)
                .ok_or(VerifierError::ChallengeNotFound)?;
        }

        self.status_tx
            .send((nonce, status))
            .await
            .map_err(|_| VerifierError::VerificationTimeout)
    }

    async fn await_status(&self, nonce: Nonce) -> Result<VerificationStatus, VerifierError> {
        let rx = self.status_rx.clone();

        loop {
            let (rx_nonce, status) = rx
                .recv()
                .await
                .map_err(|_| VerifierError::VerificationTimeout)?;
            if rx_nonce != nonce {
                continue;
            }

            return Ok(status);
        }
    }
}
