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

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(web::scope("/api").service(verifier::scope()))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
