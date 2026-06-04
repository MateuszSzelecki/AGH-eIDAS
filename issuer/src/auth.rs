use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use rusqlite::Connection;
use rand::Rng;

use crate::database;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateCodeRequest {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: u64,
}

#[derive(Serialize)]
pub struct GenerateCodeResponse {
    pub code: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub code: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[actix_web::get("/admin")]
pub async fn admin_portal() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("admin.html"))
}

#[post("/generate-code")]
pub async fn generate_code(
    db: web::Data<Mutex<Connection>>,
    req: web::Json<GenerateCodeRequest>,
) -> impl Responder {
    let mut rng = rand::thread_rng();
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let suffix: String = (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars.chars().nth(idx).unwrap()
        })
        .collect();
    let code = suffix;

    let conn = db.lock().unwrap();
    match database::create_activation_code(
        &conn,
        &code,
        &req.first_name,
        &req.last_name,
        req.date_of_birth,
    ) {
        Ok(_) => HttpResponse::Ok().json(GenerateCodeResponse { code }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[post("/register")]
pub async fn register(
    db: web::Data<Mutex<Connection>>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let conn = db.lock().unwrap();

    // 1. Verify activation code
    let code_record = match database::get_activation_code(&conn, &req.code) {
        Ok(Some(record)) => record,
        Ok(None) => return HttpResponse::BadRequest().body("Invalid activation code."),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    if code_record.used {
        return HttpResponse::BadRequest().body("This activation code has already been used.");
    }

    // 2. Hash password
    let password_hash = match bcrypt::hash(&req.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Error hashing password."),
    };

    // 3. Create user
    if let Err(e) = database::create_user(
        &conn,
        &req.username,
        &req.email,
        &password_hash,
        &code_record.first_name,
        &code_record.last_name,
        code_record.date_of_birth,
    ) {
        return HttpResponse::BadRequest().body(format!("Username is already taken or database error: {}", e));
    }

    // 4. Mark code as used
    if let Err(e) = database::mark_activation_code_used(&conn, &req.code) {
        log::warn!("Failed to mark activation code as used: {}", e);
    }

    HttpResponse::Ok().body("Registration successful")
}

#[post("/login")]
pub async fn login(
    db: web::Data<Mutex<Connection>>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let conn = db.lock().unwrap();

    let user_record = match database::get_user_by_username(&conn, &req.username) {
        Ok(Some(record)) => record,
        Ok(None) => return HttpResponse::BadRequest().body("Invalid username or password."),
        Err(e) => return HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    };

    // Verify password
    match bcrypt::verify(&req.password, &user_record.password_hash) {
        Ok(true) => {
            // For PoC session management, we generate a mock token based on the username.
            // In a production server this would be a secure signed JWT.
            let token = format!("session_token_for_{}", user_record.username);
            HttpResponse::Ok().json(LoginResponse { token })
        }
        _ => HttpResponse::BadRequest().body("Invalid username or password."),
    }
}
