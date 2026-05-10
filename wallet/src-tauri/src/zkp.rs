use crate::storage;
use crate::zkp_gen;
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

// TO DO: create real implementation
// Remember to check authentication
#[tauri::command]
pub fn request_document() -> UserDocument {
    let document = UserDocument {
        identifier: "1".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        date_of_birth: 94664820u64,
        issue_date: 1234,
        expiry_date: 1788328502,
        sig_r: "".to_string(),
        sig_s: "".to_string(),
    };
    storage::store_user_document(document.clone());
    document
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
