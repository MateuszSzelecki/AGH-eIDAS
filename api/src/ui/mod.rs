use actix_web::{HttpResponse, get, web};
use askama::Template;

#[derive(Template)]
#[template(path = "qr.html")]
struct QrTemplate;

#[derive(Template)]
#[template(path = "challenge.html")]
struct ChallengeTemplate<'a> {
    challenge: &'a str,
    qr_code: &'a str,
}

#[get("/qr")]
async fn qr() -> HttpResponse {
    QrTemplate
        .render()
        .map(|template| HttpResponse::Ok().body(template))
        .unwrap_or(HttpResponse::InternalServerError().body("Internal Server Error"))
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

    let qr_code = qrcode::QrCode::new(challenge.clone())
        .map(|code| code.render::<qrcode::render::svg::Color>().build())
        .unwrap_or("Qr code unavailable".to_string());

    ChallengeTemplate {
        qr_code: &qr_code,
        challenge: &challenge,
    }
    .render()
    .unwrap_or(challenge)
}

pub fn scope() -> actix_web::Scope {
    web::scope("/ui").service(qr).service(challenge)
}
