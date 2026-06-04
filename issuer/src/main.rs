mod database;
mod crypto;
mod auth;
mod document;

use actix_web::{App, HttpServer, middleware::Logger, web};
use std::sync::Mutex;
use std::fs;
use std::path::Path;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 1. Initialize logging
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("Starting AGH eIDAS Issuer Service...");

    // 2. Load or generate Issuer Private Key
    let assets_dir = Path::new("assets");
    if !assets_dir.exists() {
        fs::create_dir_all(assets_dir)?;
    }
    
    let sk_path = assets_dir.join("issuer_sk.bin");
    let sk_bytes = if sk_path.exists() {
        log::info!("Loading issuer private key from assets/issuer_sk.bin");
        fs::read(&sk_path)?
    } else {
        log::warn!("assets/issuer_sk.bin not found! Generating a mock 32-byte private key.");
        let dummy_sk = vec![0u8; 32];
        fs::write(&sk_path, &dummy_sk)?;
        dummy_sk
    };

    // 3. Initialize SQLite Database
    log::info!("Initializing SQLite database (issuer.db)...");
    let conn = database::init_db("issuer.db")
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let db_data = web::Data::new(Mutex::new(conn));
    let sk_data = web::Data::new(sk_bytes);

    // 4. Start HTTP Server
    log::info!("Listening on 0.0.0.0:8000");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_data.clone())
            .app_data(sk_data.clone())
            .service(auth::admin_portal)
            .service(auth::generate_code)
            .service(auth::register)
            .service(auth::login)
            .service(document::get_document)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
