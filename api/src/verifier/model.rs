use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Debug)]
pub struct Challenge {
    challenge_id: Uuid,
    pub nonce: Nonce,
    timestamp: i64,
    callback_url: String,
    verifier_name: String,
}
impl Challenge {
    pub fn new(callback_url: actix_web::http::Uri, verifier_name: &str) -> Self {
        let challenge_id = Uuid::new_v4();
        let nonce = Nonce::new();
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

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct Nonce([u8; 16]);
impl Nonce {
    fn new() -> Self {
        let mut nonce_bytes = [0u8; 16];

        use rand::RngExt;
        rand::make_rng::<rand::rngs::StdRng>().fill(&mut nonce_bytes);

        Self(nonce_bytes)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Proof {
    pub pi_a: [String; 3],
    pub pi_b: [[String; 2]; 3],
    pub pi_c: [String; 3],
    pub public_signals: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SnarkJsVerificationKey {
    pub protocol: String,
    pub curve: String,
    #[serde(rename = "nPublic")]
    pub n_public: usize,
    pub vk_alpha_1: [String; 3],
    pub vk_beta_2: [[String; 2]; 3],
    pub vk_gamma_2: [[String; 2]; 3],
    pub vk_delta_2: [[String; 2]; 3],
    pub vk_alphabeta_12: [[[String; 2]; 3]; 2],
    #[serde(rename = "IC")]
    pub ic: Vec<[String; 3]>,
}
