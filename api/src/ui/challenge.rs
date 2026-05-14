use actix_web::get;
use askama::Template;

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
<style>
    .instruction {
        margin: 15px 0;
        font-size: 1.1em;
        color: #555;
    }
    .challenge-details {
        margin-top: 20px;
        padding: 15px;
        background-color: #f8f9fa;
        border-left: 4px solid #667eea;
        text-align: left;
    }
    .challenge-details h3 {
        margin-top: 0;
        color: #667eea;
    }
    .challenge-details pre {
        background-color: #fff;
        padding: 10px;
        border-radius: 4px;
        overflow-x: auto;
        font-size: 0.9em;
    }
    svg {
        margin: 20px 0;
        border: 2px solid #667eea;
        border-radius: 8px;
    }
    .status-container {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 12px;
        margin-top: 25px;
        padding: 15px;
        border-top: 1px solid #eee;
        color: #667eea;
        font-weight: 500;
    }
    .spinner {
        width: 20px;
        height: 20px;
        border: 3px solid rgba(102, 126, 234, 0.3);
        border-radius: 50%;
        border-top-color: #667eea;
        animation: spin 1s ease-in-out infinite;
    }
    @keyframes spin {
        to { transform: rotate(360deg); }
    }
    .pulse {
        width: 8px;
        height: 8px;
        background-color: #667eea;
        border-radius: 50%;
        box-shadow: 0 0 0 rgba(102, 126, 234, 0.4);
        animation: pulse 2s infinite;
    }
    @keyframes pulse {
        0% { box-shadow: 0 0 0 0 rgba(102, 126, 234, 0.4); }
        70% { box-shadow: 0 0 0 10px rgba(102, 126, 234, 0); }
        100% { box-shadow: 0 0 0 0 rgba(102, 126, 234, 0); }
    }
</style>

{{ qr_code | safe }}
<p class="instruction">📱 Zeskanuj kod QR aplikacją mobilną</p>

<div class="challenge-details">
    <h3>📋 Szczegóły Challenge:</h3>
    <pre>{{ challenge }}</pre>
</div>

<div id="verification-status" 
     hx-get="/ui/status?nonce={{ nonce }}" 
     hx-trigger="load" 
     hx-target="#container" 
     hx-swap="innerHTML"
     class="status-container">
    <div class="spinner"></div>
    <span>Oczekiwanie na weryfikację...</span>
    <div class="pulse"></div>
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
