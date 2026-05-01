use actix_web::http::Uri;

use crate::verifier::model::Challenge;

pub fn generate_challenge() -> Challenge {
    let callback_url = Uri::from_static("https://127.0.0.1:3000/api/verifier/verify");
    Challenge::new(callback_url, "TestVerifier")
}
