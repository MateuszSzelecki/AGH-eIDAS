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
pub fn login(
    state: tauri::State<Mutex<User>>,
    username: String,
    password: String,
) -> Result<(), Error> {
    // TO DO: validate data
    // TO DO: get real token from api
    //
    //
    let token = "test".to_string();
    let mut user = state.lock().unwrap();

    user.set_token(token);

    Ok(())
}

#[tauri::command]
pub fn register(
    state: tauri::State<Mutex<User>>,
    username: String,
    password: String,
) -> Result<(), Error> {
    // TO DO: validate data
    // TO DO: implement register logic
    //
    //

    Ok(())
}

#[tauri::command]
pub fn logout(state: tauri::State<Mutex<User>>) -> Result<(), Error> {
    // TO DO: ensure that it deletes everything it needs to
    let mut user = state.lock().unwrap();
    user.set_token(String::new());
    storage::delete_token();
    Ok(())
}
