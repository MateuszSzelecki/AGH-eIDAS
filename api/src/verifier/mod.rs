mod controller;
mod model;
mod service;

use std::{
    collections::HashMap,
    fmt::Display,
    sync::{Arc, Mutex, MutexGuard},
};

pub use controller::scope;
pub use service::expire_challenges;

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

#[derive(Debug)]
enum VerificationStatus {
    Success,
    Failure,
    Expired,
}
pub struct VerifierData {
    challenges: Arc<Mutex<HashMap<Nonce, Challenge>>>,
    status_tx: kanal::AsyncSender<(Nonce, VerificationStatus)>,
    status_rx: kanal::AsyncReceiver<(Nonce, VerificationStatus)>,
    vk: SnarkJsVerificationKey,
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

    fn challenges<'a>(&'a self) -> Option<MutexGuard<'a, HashMap<Nonce, Challenge>>> {
        self.challenges.lock().ok()
    }
}
