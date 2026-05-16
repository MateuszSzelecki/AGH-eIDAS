use actix_web::{
    HttpResponse, get, post,
    web::{self, Data, Json, Query},
};
use serde::{Deserialize, Serialize};

use crate::verifier::{
    VerificationStatus, VerifierData, VerifierError,
    model::{Nonce, PublicInputs},
    service::{handle_generate_challenge, handle_proof_verification, handle_verification_status},
};

#[get("/challenge")]
async fn challenge(verifier_data: Data<VerifierData>) -> Result<HttpResponse, VerifierError> {
    handle_generate_challenge(&verifier_data).map(|challenge| HttpResponse::Ok().json(challenge))
}

#[derive(Clone, Deserialize, Debug)]
struct VerificationRequest {
    proof: String,
    public_inputs: PublicInputs,
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

    Json(VerificationRequest {
        proof,
        public_inputs,
    }): Json<VerificationRequest>,
) -> Result<HttpResponse, VerifierError> {
    handle_proof_verification(&verifier_data, &proof, public_inputs)
        .await
        .map(|is_valid| {
            if is_valid {
                HttpResponse::Ok().json(VerificationResponse::success())
            } else {
                HttpResponse::BadRequest().json(VerificationResponse::failure())
            }
        })
}

#[derive(Deserialize)]
struct StatusQuery {
    #[serde(deserialize_with = "deserialize_nonce_string")]
    nonce: Nonce,
}
fn deserialize_nonce_string<'de, D>(deserializer: D) -> Result<Nonce, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(serde::de::Error::custom)
}
#[get("/status")]
async fn status(
    verifier_data: Data<VerifierData>,

    Query(StatusQuery { nonce }): Query<StatusQuery>,
) -> Result<HttpResponse, VerifierError> {
    handle_verification_status(&verifier_data, nonce)
        .await
        .map(|status| {
            HttpResponse::Ok().body(match status {
                VerificationStatus::Success => "success",
                VerificationStatus::Failure => "failure",
                VerificationStatus::Expired => "expired",
            })
        })
}

pub fn scope() -> actix_web::Scope {
    web::scope("/verifier")
        .service(challenge)
        .service(verify)
        .service(status)
}
