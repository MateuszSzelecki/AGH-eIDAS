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

use crate::verifier::model::{Challenge, Nonce};

#[derive(Debug, thiserror::Error)]
pub enum VerifierError {
    StoreChallengeFailed,
    ChallengeNotFound,
    VerificationFailed,
    VerificationTimeout,
}

impl actix_web::error::ResponseError for VerifierError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            VerifierError::StoreChallengeFailed => {
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
pub enum VerificationStatus {
    Success,
    Failure,
    Expired,
}
pub struct VerifierData {
    challenges: Arc<Mutex<HashMap<Nonce, Challenge>>>,
    status_tx: kanal::AsyncSender<(Nonce, VerificationStatus)>,
    status_rx: kanal::AsyncReceiver<(Nonce, VerificationStatus)>,
}
impl VerifierData {
    pub fn new() -> Self {
        let status_channel = kanal::unbounded_async();
        Self {
            challenges: Arc::new(Mutex::new(HashMap::new())),
            status_tx: status_channel.0,
            status_rx: status_channel.1,
        }
    }

    fn challenges<'a>(&'a self) -> Option<MutexGuard<'a, HashMap<Nonce, Challenge>>> {
        self.challenges.lock().ok()
    }

    #[cfg(test)]
    pub fn insert_test_challenge(&self, nonce: u128) {
        let nonce = Nonce::from_bytes(nonce.to_be_bytes());
        let mut challenge = Challenge::new(
            actix_web::http::Uri::from_static("http://test/verify"),
            "test",
        );
        challenge.nonce = nonce;
        self.challenges()
            .expect("test challenge mutex must lock")
            .insert(nonce, challenge);
    }
}
