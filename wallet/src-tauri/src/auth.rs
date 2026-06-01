use log::info;
use std::sync::Mutex;
use tauri_plugin_biometric::{AuthOptions, BiometricExt, Error};

use crate::storage;

pub struct User {
    token: String,
}

impl User {
    pub fn new() -> Self {
        Self {
            token: String::new(),
        }
    }

    fn set_token(&mut self, token: String) -> bool {
        //TO DO: Verify token than set it
        self.token = token.clone();
        storage::store_token(&token);
        return true;
    }

    fn get_token(&mut self, app_handle: tauri::AppHandle) -> String {
        if !self.token.is_empty() {
            return self.token.clone();
        }

        if storage::check_token_existence() {
            // Copied from tauri docs i think
            match self.biometric_check(app_handle) {
                Ok(_) => {
                    println!("Hooray! Successfully Authenticated! We can now perform the locked Tauri function!");
                }
                Err(e) => {
                    println!("Oh no! Authentication failed because : {e}");
                    return String::from("");
                }
            };
            let token: String = match storage::get_token() {
                Ok(s) => {
                    println!("Token found in keystore");
                    s
                }
                Err(e) => {
                    println!("No token found");
                    return String::from("");
                }
            };
            self.set_token(token.clone());
            info!("{:?}", token);
            return token;
        }
        "".to_string()
    }

    fn biometric_check(&self, app_handle: tauri::AppHandle) -> Result<(), Error> {
        //TO DO: check if biometric is avaible
        let options = AuthOptions {
            // Set True if you want the user to be able to authenticate using phone password
            allow_device_credential: false,
            cancel_title: Some("Feature won't work if Canceled".to_string()),
            fallback_title: Some("Sorry, authentication failed".to_string()),

            title: Some("Tauri feature".to_string()),
            subtitle: Some("Authenticate to access the locked Tauri function".to_string()),
            confirmation_required: Some(true),
        };

        app_handle
            .biometric()
            .authenticate("BASE PROMPT".to_string(), options)
    }
}

#[tauri::command]
pub fn is_auth(state: tauri::State<Mutex<User>>, app_handle: tauri::AppHandle) -> bool {
    let mut user = state.lock().unwrap();

    // TO DO: here validate token
    if !user.get_token(app_handle).is_empty() {
        return true;
    }

    return false;
}

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, Mutex<User>>,
    username: String,
    password: String,
    issuer_url: String,
) -> Result<(), String> {
    info!("Logging in user: {}", username);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|e| e.to_string())?;

    let payload = serde_json::json!({
        "username": username,
        "password": password
    });

    let token = match client
        .post(format!("{}/login", issuer_url))
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
                body.get("token")
                    .or_else(|| body.get("accessToken"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| "No token found in response".to_string())?
            } else {
                return Err(format!("Login failed with status: {}", response.status()));
            }
        }
        Err(err) => {
            log::warn!("Could not connect to Issuer API: {}. Using offline mock mode.", err);
            format!("mock_token_for_{}", username)
        }
    };

    let mut user = state.lock().unwrap();
    user.set_token(token);

    Ok(())
}

fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let domain = parts[1];
    if !domain.contains('.') {
        return false;
    }
    let local = parts[0];
    if local.is_empty() || domain.is_empty() || domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }
    true
}

#[tauri::command]
pub async fn register(
    username: String,
    email: String,
    office_code: String,
    password: String,
    issuer_url: String,
) -> Result<(), String> {
    info!("Registering user: {}", username);

    if !is_valid_email(&email) {
        return Err("Niepoprawny format adresu e-mail.".to_string());
    }

    if password.len() < 8
        || !password.chars().any(|c| c.is_uppercase())
        || !password.chars().any(|c| c.is_lowercase())
        || !password.chars().any(|c| c.is_numeric())
    {
        return Err("Hasło musi mieć co najmniej 8 znaków, wielką i małą literę oraz cyfrę.".to_string());
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|e| e.to_string())?;

    let payload = serde_json::json!({
        "username": username,
        "email": email,
        "code": office_code,
        "password": password
    });

    match client
        .post(format!("{}/register", issuer_url))
        .json(&payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                Ok(())
            } else {
                Err(format!("Registration failed with status: {}", response.status()))
            }
        }
        Err(err) => {
            log::warn!("Could not connect to Issuer API: {}. Using offline mock mode.", err);
            Ok(())
        }
    }
}

#[tauri::command]
pub fn logout(state: tauri::State<Mutex<User>>) -> Result<(), Error> {
    // TO DO: ensure that it deletes everything it needs to
    let mut user = state.lock().unwrap();
    user.set_token(String::new());
    storage::delete_token();
    Ok(())
}
