use actix_web::{get, web::Query};
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
<style>
    .checkmark {
        font-size: 80px;
        animation: pop 0.4s ease-out;
    }
    @keyframes pop {
        0%   { transform: scale(0); }
        80%  { transform: scale(1.15); }
        100% { transform: scale(1); }
    }
    h1 {
        font-size: 1.8em;
        color: #2e7d32;
        margin: 20px 0 10px;
    }
    p {
        color: #555;
        font-size: 1.05em;
        line-height: 1.6;
    }
    .new-btn {
        display: inline-block;
        margin-top: 30px;
        padding: 12px 28px;
        background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
        color: #1b5e20;
        font-weight: bold;
        text-decoration: none;
        border-radius: 8px;
        transition: opacity 0.2s;
        cursor: pointer;
        border: none;
    }
    .new-btn:hover { opacity: 0.85; }
</style>

<div class="checkmark">✅</div>
<h1>Weryfikacja przebiegła pomyślnie!</h1>
<p>Tożsamość oraz spełnienie warunków zostały potwierdzone z użyciem Zero-Knowledge Proof.</p>
<button class="new-btn" hx-get="/ui/verification" hx-target="#container" hx-swap="innerHTML">
    Wygeneruj nowy Challenge
</button>
"##
)]
struct SuccessTemplate;

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
<style>
    .crossmark {
        font-size: 80px;
        animation: pop 0.4s ease-out;
    }
    @keyframes pop {
        0%   { transform: scale(0); }
        80%  { transform: scale(1.15); }
        100% { transform: scale(1); }
    }
    h1 {
        font-size: 1.8em;
        color: #c62828;
        margin: 20px 0 10px;
    }
    p {
        color: #555;
        font-size: 1.05em;
        line-height: 1.6;
    }
    .new-btn {
        display: inline-block;
        margin-top: 30px;
        padding: 12px 28px;
        background: linear-gradient(135deg, #ff5f6d 0%, #ffc371 100%);
        color: #7f0000;
        font-weight: bold;
        text-decoration: none;
        border-radius: 8px;
        transition: opacity 0.2s;
        cursor: pointer;
        border: none;
    }
    .new-btn:hover { opacity: 0.85; }
</style>

<div class="crossmark">❌</div>
<h1>Weryfikacja nie powiodła się</h1>
<p>Tożsamość lub warunki nie mogły zostać potwierdzone. Spróbuj ponownie lub skontaktuj się z administratorem.</p>
<button class="new-btn" hx-get="/ui/verification" hx-target="#container" hx-swap="innerHTML">
    Spróbuj ponownie
</button>
"##
)]
struct FailureTemplate;

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
<style>
    .expired-icon {
        font-size: 80px;
        animation: pop 0.4s ease-out;
    }
    @keyframes pop {
        0%   { transform: scale(0); }
        80%  { transform: scale(1.15); }
        100% { transform: scale(1); }
    }
    h1 {
        font-size: 1.8em;
        color: #e65100;
        margin: 20px 0 10px;
    }
    p {
        color: #555;
        font-size: 1.05em;
        line-height: 1.6;
    }
    .new-btn {
        display: inline-block;
        margin-top: 30px;
        padding: 12px 28px;
        background: linear-gradient(135deg, #ff9800 0%, #ffc107 100%);
        color: #5d4037;
        font-weight: bold;
        text-decoration: none;
        border-radius: 8px;
        transition: opacity 0.2s;
        cursor: pointer;
        border: none;
    }
    .new-btn:hover { opacity: 0.85; }
</style>

<div class="expired-icon">⏳</div>
<h1>Challenge wygasł</h1>
<p>Sesja wygasła z powodu braku aktywności. Wygeneruj nowy kod QR, aby kontynuować.</p>
<button class="new-btn" hx-get="/ui/verification" hx-target="#container" hx-swap="outerHTML">
    Odśwież Challenge
</button>
"##
)]
struct ExpiredTemplate;

#[derive(Deserialize)]
struct StatusQuery {
    nonce: String,
}
#[get("/status")]
pub async fn status(Query(StatusQuery { nonce }): Query<StatusQuery>) -> String {
    async fn get_status(nonce: &str) -> String {
        let client = reqwest::Client::new();
        client
            .get("http://127.0.0.1:3000/api/verifier/status")
            .query(&[("nonce", nonce)])
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap_or_else(|_| "failure".to_string())
    }

    match get_status(&nonce).await.as_str() {
        "success" => SuccessTemplate
            .render()
            .unwrap_or("Verification success!".to_string()),
        "expired" => ExpiredTemplate
            .render()
            .unwrap_or("Challenge expired!".to_string()),
        _ => FailureTemplate
            .render()
            .unwrap_or("Verification failed!".to_string()),
    }
}
