mod ui;
mod verifier;

use actix_web::{App, HttpServer, web};

fn init_logging() {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging();

    let verifier_data = web::Data::new(verifier::VerifierData::new());

    // Cleanup expired challenges
    let cleanup_verifier_data = verifier_data.clone();
    actix_web::rt::spawn(async move {
        loop {
            log::info!("Cleanig up expired challenges");
            actix_web::rt::time::sleep(std::time::Duration::from_mins(1)).await;
            verifier::expire_challenges(&cleanup_verifier_data).await;
        }
    });

    HttpServer::new(move || {
        let worker_verifier_data = verifier_data.clone();

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(web::scope("/api").service(verifier::scope().app_data(worker_verifier_data)))
            .service(ui::scope())
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
