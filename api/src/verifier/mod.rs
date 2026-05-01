mod controller;
mod model;
mod service;

use std::{collections::HashMap, fmt::Display, sync::Mutex};

pub use controller::scope;

use crate::verifier::model::{Challenge, Nonce};

#[derive(Debug, thiserror::Error)]
enum VerifierError {
    StoreChallengeFailed,
    ChallengeNotFound,
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
        }
    }
}

struct VerifierData {
    challenges: Mutex<HashMap<Nonce, Challenge>>,
}
impl VerifierData {
    fn new() -> Self {
        Self {
            challenges: Mutex::new(HashMap::new()),
        }
    }

    fn store_challenge(&self, challenge: &Challenge) -> Result<(), VerifierError> {
        let mut challenges = self
            .challenges
            .lock()
            .map_err(|_| VerifierError::StoreChallengeFailed)?;
        challenges.insert(challenge.nonce.clone(), challenge.clone());
        Ok(())
    }

    fn get_challenge(&self, nonce: &Nonce) -> Option<Challenge> {
        let challenges = self.challenges.lock().ok()?;
        challenges.get(nonce).cloned()
    }
}
