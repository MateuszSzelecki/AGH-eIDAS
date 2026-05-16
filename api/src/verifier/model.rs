use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use serde::de::Error as DeError;

const EXPITY_PERIOD: chrono::TimeDelta = chrono::TimeDelta::minutes(5);

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    challenge_id: Uuid,
    pub nonce: Nonce,
    timestamp: i64,
    expiry: i64,
    callback_url: String,
    verifier_name: String,
}
impl Challenge {
    pub fn new(callback_url: actix_web::http::Uri, verifier_name: &str) -> Self {
        let challenge_id = Uuid::new_v4();
        let nonce = Nonce::new();
        let generation_time = chrono::Utc::now();
        let timestamp = generation_time.timestamp() / 1000;
        let expiry = generation_time
            .checked_add_signed(EXPITY_PERIOD)
            .expect("Timestamp must be in range")
            .timestamp();
        Self {
            challenge_id,
            nonce,
            timestamp,
            expiry,
            callback_url: callback_url.to_string(),
            verifier_name: verifier_name.to_string(),
        }
    }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().timestamp() > self.expiry
    }
}

#[derive(Default, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct Nonce(#[serde(serialize_with = "to_hex", deserialize_with = "from_hex")] [u8; 16]);
impl Nonce {
    fn new() -> Self {
        let mut nonce_bytes = [0u8; 16];

        use rand::RngExt;
        rand::make_rng::<rand::rngs::StdRng>().fill(&mut nonce_bytes);

        Self(nonce_bytes)
    }

    pub fn from_bytes(nonce_bytes: [u8; 16]) -> Self {
        Self(nonce_bytes)
    }
}

// Vibecoded function for testing, To be checked
fn to_hex<S>(bytes: &[u8; 16], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut s = String::with_capacity(32);

    for b in bytes {
        use std::fmt::Write;
        write!(&mut s, "{:02x}", b).unwrap();
    }

    serializer.serialize_str(&s)
}

fn from_hex<'de, D>(deserializer: D) -> Result<[u8; 16], D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if s.len() != 32 {
        return Err(D::Error::custom("expected 32 hex characters"));
    }

    let mut out = [0u8; 16];

    for i in 0..16 {
        let byte_str = &s[i * 2..i * 2 + 2];
        out[i] = u8::from_str_radix(byte_str, 16).map_err(D::Error::custom)?;
    }

    Ok(out)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicInputs {
    pub generation_date: u64,
    pub nonce: u128,
}
