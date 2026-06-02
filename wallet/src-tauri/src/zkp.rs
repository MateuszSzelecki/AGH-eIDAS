use crate::storage;
use crate::zkp_gen;
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Mutex;

use crate::auth::User;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChallengePayload {
    challenge_id: String,
    nonce: String,
    timestamp: u64,
    callback_url: String,
    verifier_name: String,
}

impl ChallengePayload {
    pub fn get_nonce(&self) -> u128 {
        let nonce_u128 = u128::from_str_radix(self.nonce.trim_start_matches("0x"), 16).unwrap();
        nonce_u128
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDocument {
    pub identifier: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: u64,
    pub issue_date: u64,
    pub expiry_date: u64,
    pub sig_r: String,
    pub sig_s: String,
}

#[derive(Serialize, Deserialize)]
struct PublicInputs {
    generation_date: u64,
    nonce: u128,
}

impl PublicInputs {
    fn new(generation_date: u64, nonce: u128) -> Self {
        Self {
            generation_date,
            nonce,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProofPayload {
    proof: String,
    public_inputs: PublicInputs,
}

impl ProofPayload {
    pub fn new(proof: String, generation_date: u64, nonce: u128) -> Self {
        Self {
            proof,
            public_inputs: PublicInputs::new(generation_date, nonce),
        }
    }
}

#[derive(Serialize)]
struct VerificationRequest {
    nonce: String,
    proof: Value,
}

/// Fetches a signed UserDocument from the Issuer API using the stored session token.
/// The issuer_url is resolved from the frontend (same hostname, port 8000).
#[tauri::command]
pub async fn request_document(
    state: tauri::State<'_, Mutex<User>>,
    issuer_url: String,
) -> Result<UserDocument, String> {
    let token = {
        let user = state.lock().unwrap();
        user.get_token_value()
    };

    if token.is_empty() {
        return Err("No authorization token. Please log in again.".to_string());
    }

    info!("Requesting document from Issuer API: {}/document", issuer_url);

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get(format!("{}/document", issuer_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Could not connect to Issuer API: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("Issuer API returned error {}: {}", status, body));
    }

    let document: UserDocument = response
        .json()
        .await
        .map_err(|e| format!("Error parsing response: {}", e))?;

    // Persist the document locally for offline access
    if let Err(e) = storage::store_user_document(document.clone()) {
        log::warn!("Failed to persist document locally: {}", e);
    }

    Ok(document)
}

#[tauri::command]
pub fn load_document() -> UserDocument {
    let document: UserDocument = match storage::get_user_document() {
        Ok(s) => s,
        Err(_) => UserDocument {
            identifier: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            date_of_birth: 0,
            issue_date: 0,
            expiry_date: 0,
            sig_r: "".to_string(),
            sig_s: "".to_string(),
        },
    };

    document
}

#[tauri::command]
pub async fn generate_proof(challenge: ChallengePayload) -> Result<(), String> {
    info!("GENERATING PROOF");
    inner_generate_proof(challenge)
        .await
        .map_err(|e| e.to_string())
}

async fn inner_generate_proof(
    challenge: ChallengePayload,
) -> Result<(), Box<dyn std::error::Error>> {
    // TO DO: Verify Challenge
    // Right now it automatically sends proof
    let user_document = storage::get_user_document()?;

    info!("STARTING PROOF GENERATION");
    let proof = zkp_gen::generate_proof(user_document, challenge.clone())?;
    info!("PROOF GENERATED SENDING");
    send_proof(proof, challenge).await?;
    info!("PROOF SENT");
    Ok(())
}

async fn send_proof(
    proof: ProofPayload,
    challenge: ChallengePayload,
) -> Result<(), Box<dyn std::error::Error>> {
    // REMEMBER TO CHECK OF ON THE API SIDE IF NONCE FROM PUBLIC INPUTS IS CHECKED!!!!
    let client = Client::new();

    let res = client
        .post(&challenge.callback_url)
        .json(&proof)
        .send()
        .await?;

    info!("Status: {:?}", res);

    Ok(())
}
