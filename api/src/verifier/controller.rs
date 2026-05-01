use actix_web::{
    HttpResponse, get, post,
    web::{self, Data, Json},
};
use serde::{Deserialize, Serialize};

use crate::verifier::{
    VerifierData, VerifierError,
    model::{Nonce, Proof},
    service::{generate_challenge, verify_proof},
};

#[get("/challenge")]
async fn challenge(verifier_data: Data<VerifierData>) -> Result<HttpResponse, VerifierError> {
    log::info!("Requested a challenge");

    let challenge = generate_challenge();

    log::info!("Generated challenge: {challenge:?}");

    verifier_data.store_challenge(&challenge)?;

    log::info!("Stored challenge for verification");

    Ok(HttpResponse::Ok().json(challenge))
}

#[derive(Clone, Deserialize, Debug)]
struct VerificationRequest {
    proof: Proof,
    nonce: Nonce,
}
#[derive(Clone, Serialize, Debug)]
struct VerificationResponse {
    success: bool,
    message: String,
}
impl VerificationResponse {
    fn success() -> Self {
        Self {
            success: true,
            message: "Verification successful".to_string(),
        }
    }
    fn failure() -> Self {
        Self {
            success: false,
            message: "Verification failed".to_string(),
        }
    }
}
#[post("/verify")]
async fn verify(
    verifier_data: Data<VerifierData>,

    Json(VerificationRequest { nonce, proof }): Json<VerificationRequest>,
) -> Result<HttpResponse, VerifierError> {
    log::info!("Requested a verification for {nonce:?} and {proof:?}");

    if verify_proof(proof, nonce) {
        log::info!("Verification successful");
        Ok(HttpResponse::Ok().json(VerificationResponse::success()))
    } else {
        log::info!("Verification failed");
        Ok(HttpResponse::BadRequest().json(VerificationResponse::failure()))
    }
}

pub fn scope() -> actix_web::Scope {
    web::scope("/verifier")
        .app_data(web::Data::new(VerifierData::new()))
        .service(challenge)
        .service(verify)
}
