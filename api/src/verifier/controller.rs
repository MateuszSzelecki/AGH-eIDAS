use actix_web::{HttpResponse, get, web};

use crate::verifier::{model::VerifierError, service::generate_challenge};

#[get("/challenge")]
async fn generate_qr_code() -> Result<HttpResponse, VerifierError> {
    log::info!("Requested a challenge");

    let challenge = generate_challenge();

    log::info!("Generated challenge: {challenge:?}");

    Ok(HttpResponse::Ok().json(challenge))
}

pub fn scope() -> actix_web::Scope {
    web::scope("/verifier").service(generate_qr_code)
}
