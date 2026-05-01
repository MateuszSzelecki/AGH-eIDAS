use actix_web::{HttpResponse, get, web};

use crate::verifier::{VerifierData, VerifierError, service::generate_challenge};

#[get("/challenge")]
async fn generate_qr_code(
    verifier_data: web::Data<VerifierData>,
) -> Result<HttpResponse, VerifierError> {
    log::info!("Requested a challenge");

    let challenge = generate_challenge();

    log::info!("Generated challenge: {challenge:?}");

    verifier_data.store_challenge(&challenge)?;

    log::info!("Stored challenge for verification");

    Ok(HttpResponse::Ok().json(challenge))
}

pub fn scope() -> actix_web::Scope {
    web::scope("/verifier")
        .app_data(web::Data::new(VerifierData::new()))
        .service(generate_qr_code)
}
