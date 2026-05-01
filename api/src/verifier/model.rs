use uuid::Uuid;

use crate::verifier::service::generate_nonce;

#[derive(Clone, serde::Serialize, Debug)]
pub struct Challenge {
    challenge_id: Uuid,
    nonce: [u8; 16],
    timestamp: i64,
    callback_url: String,
    verifier_name: String,
}
impl Challenge {
    pub fn new(callback_url: actix_web::http::Uri, verifier_name: &str) -> Self {
        let challenge_id = Uuid::new_v4();
        let nonce = generate_nonce();
        let timestamp = chrono::Utc::now().timestamp() / 1000;
        Self {
            challenge_id,
            nonce,
            timestamp,
            callback_url: callback_url.to_string(),
            verifier_name: verifier_name.to_string(),
        }
    }
}
