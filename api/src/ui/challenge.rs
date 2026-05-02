use actix_web::get;
use askama::Template;

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
{{ qr_code | safe }}
<p class="instruction">📱 Zeskanuj kod QR aplikacją mobilną</p>

<div class="challenge-details">
    <h3>📋 Szczegóły Challenge:</h3>
    <pre>{{ challenge }}</pre>
</div>

<div id="verification-status" hx-get="/ui/status?nonce={{ nonce }}" hx-trigger="load" hx-target="#container" hx-swap="outerhtml">
  Waiting for verification...
</div>
"##
)]
struct ChallengeTemplate<'a> {
    challenge: &'a str,
    nonce: &'a str,
    qr_code: &'a str,
}

#[get("/challenge")]
pub async fn challenge() -> String {
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
