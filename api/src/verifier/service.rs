use actix_web::http::Uri;
use rand::RngExt;

use crate::verifier::model::Challenge;

pub fn generate_challenge() -> Challenge {
    let callback_url = Uri::from_static("https://127.0.0.1:3000/api/verifier/verify");
    Challenge::new(callback_url, "TestVerifier")
}

pub fn generate_nonce() -> [u8; 16] {
    let mut nonce_bytes = [0u8; 16];
    rand::make_rng::<rand::rngs::StdRng>().fill(&mut nonce_bytes);
    nonce_bytes
}
