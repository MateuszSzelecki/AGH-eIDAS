use actix_web::{
    HttpResponse, get,
    web::{self, Query},
};
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "qr.html")]
struct QrTemplate;

#[get("/qr")]
async fn qr() -> HttpResponse {
    QrTemplate
        .render()
        .map(|template| HttpResponse::Ok().body(template))
        .unwrap_or(HttpResponse::InternalServerError().body("Internal Server Error"))
}

#[derive(Template)]
#[template(path = "challenge.html")]
struct ChallengeTemplate<'a> {
    challenge: &'a str,
    nonce: &'a str,
    qr_code: &'a str,
}

#[get("/challenge")]
async fn challenge() -> String {
    async fn get_challenge() -> Option<String> {
        Some(
            reqwest::get("http://127.0.0.1:3000/api/verifier/challenge")
                .await
                .ok()?
                .text()
                .await
                .ok()?
                .to_string(),
        )
    }

    let Some(challenge) = get_challenge().await else {
        return "Failed to generate a challenge!".to_string();
    };

    let nonce = serde_json::from_str::<serde_json::Value>(&challenge.clone())
        .map(|v| v["nonce"].to_string())
        .unwrap_or_default();

    let qr_code = qrcode::QrCode::new(challenge.clone())
        .map(|code| code.render::<qrcode::render::svg::Color>().build())
        .unwrap_or("Qr code unavailable".to_string());

    ChallengeTemplate {
        challenge: &challenge,
        nonce: &nonce,
        qr_code: &qr_code,
    }
    .render()
    .unwrap_or(challenge)
}

#[derive(Template)]
#[template(path = "success.html")]
struct SuccessTemplate;

#[derive(Template)]
#[template(path = "failure.html")]
struct FailureTemplate;

#[derive(Deserialize)]
struct StatusQuery {
    nonce: String,
}
#[get("/status")]
async fn status(Query(StatusQuery { nonce }): Query<StatusQuery>) -> String {
    async fn get_status(nonce: &str) -> Option<String> {
        let client = reqwest::Client::new();
        Some(
            client
                .get("http://127.0.0.1:3000/api/verifier/status")
                .query(&[("nonce", nonce)])
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap()
                .to_string(),
        )
    }

    if get_status(&nonce).await.unwrap_or("failure".to_string()) == "success" {
        SuccessTemplate
            .render()
            .unwrap_or("Verification success!".to_string())
    } else {
        FailureTemplate
            .render()
            .unwrap_or("Verification failed!".to_string())
    }
}

pub fn scope() -> actix_web::Scope {
    web::scope("/ui")
        .service(qr)
        .service(challenge)
        .service(status)
}
