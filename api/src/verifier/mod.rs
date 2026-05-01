mod controller;
mod model;
mod service;

use std::{collections::HashMap, fmt::Display, sync::Mutex};

pub use controller::scope;

use crate::verifier::model::{Challenge, Nonce};

#[derive(Debug, thiserror::Error)]
enum VerifierError {
    StoreChallengeFailed,
}
impl actix_web::error::ResponseError for VerifierError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            VerifierError::StoreChallengeFailed => {
                actix_web::HttpResponse::InternalServerError().body("Internal Server Error")
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
}
