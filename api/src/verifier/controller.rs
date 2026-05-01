use actix_web::{
    HttpResponse, get, post,
    web::{self, Data, Json},
};
use serde::{Deserialize, Serialize};

use crate::verifier::{
    VerifierData, VerifierError,
    model::{Nonce, Proof},
    service::{handle_generate_challenge, handle_proof_verification},
};

#[get("/challenge")]
async fn challenge(verifier_data: Data<VerifierData>) -> Result<HttpResponse, VerifierError> {
    handle_generate_challenge(&verifier_data).map(|challenge| HttpResponse::Ok().json(challenge))
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
    handle_proof_verification(&verifier_data, &nonce, &proof).map(|is_valid| {
        if is_valid {
            HttpResponse::Ok().json(VerificationResponse::success())
        } else {
            HttpResponse::BadRequest().json(VerificationResponse::failure())
        }
    })
}

pub fn scope() -> actix_web::Scope {
    web::scope("/verifier")
        .app_data(web::Data::new(VerifierData::new()))
        .service(challenge)
        .service(verify)
}
