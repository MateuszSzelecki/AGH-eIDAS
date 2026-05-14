use std::sync::Mutex;
use tauri_plugin_log::{Target, TargetKind};
mod auth;
mod storage;
mod zkp;
mod zkp_gen;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .setup(|_app| {
            storage::init_keyring().expect("Failed to initate Android Keystore");
            Ok(())
        })
        .plugin(tauri_plugin_biometric::init())
        .plugin(tauri_plugin_barcode_scanner::init())
        .manage(Mutex::new(auth::User::new()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            auth::is_auth,
            auth::login,
            auth::register,
            auth::logout,
            zkp::request_document,
            zkp::load_document,
            zkp::generate_proof,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
