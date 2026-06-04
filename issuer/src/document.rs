use actix_web::{get, web, HttpResponse, Responder, HttpRequest};
use serde::Serialize;
use std::sync::Mutex;
use rusqlite::Connection;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::database;
use crate::crypto;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct UserDocument {
    pub identifier: String,
    pub firstName: String,
    pub lastName: String,
    pub dateOfBirth: u64,
    pub issueDate: u64,
    pub expiryDate: u64,
    pub sigR: String,
    pub sigS: String,
}

#[get("/document")]
pub async fn get_document(
    db: web::Data<Mutex<Connection>>,
    req: HttpRequest,
    sk_bytes: web::Data<Vec<u8>>,
) -> impl Responder {
    // 1. Get Authorization header
    let auth_header = match req.headers().get("Authorization") {
        Some(val) => match val.to_str() {
            Ok(s) => s,
            Err(_) => return HttpResponse::Unauthorized().body("Invalid token format."),
        },
        None => return HttpResponse::Unauthorized().body("Missing Authorization header."),
    };

    // Expecting format: "Bearer session_token_for_<username>"
    let token_prefix = "Bearer session_token_for_";
    if !auth_header.starts_with(token_prefix) {
        return HttpResponse::Unauthorized().body("Invalid authorization token.");
    }

    let username = &auth_header[token_prefix.len()..];
    if username.is_empty() {
        return HttpResponse::Unauthorized().body("Missing username in token.");
    }

    let conn = db.lock().unwrap();

    // 2. Fetch user from DB
    let user_record = match database::get_user_by_username(&conn, username) {
        Ok(Some(record)) => record,
        Ok(None) => return HttpResponse::NotFound().body("User does not exist."),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    // 3. Cryptographically sign date of birth
    let (sig_r, sig_s) = match crypto::sign_birthdate(&sk_bytes, user_record.date_of_birth) {
        Ok(sigs) => sigs,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Cryptographic error: {}", e)),
    };

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let document = UserDocument {
        identifier: uuid::Uuid::new_v4().to_string(),
        firstName: user_record.first_name,
        lastName: user_record.last_name,
        dateOfBirth: user_record.date_of_birth,
        issueDate: now,
        expiryDate: now + 30 * 24 * 60 * 60, // 1 month expiration
        sigR: sig_r,
        sigS: sig_s,
    };

    HttpResponse::Ok().json(document)
}
