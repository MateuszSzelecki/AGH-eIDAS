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
    }
    .new-btn:hover { opacity: 0.85; }
</style>

<div class="checkmark">✅</div>
<h1>Weryfikacja przebiegła pomyślnie!</h1>
<p>Tożsamość oraz spełnienie warunków zostały potwierdzone z użyciem Zero-Knowledge Proof.</p>
<a class="new-btn" href="/ui/qr">Wygeneruj nowy Challenge</a>
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
    }
    .new-btn:hover { opacity: 0.85; }
</style>

<div class="crossmark">❌</div>
<h1>Weryfikacja nie powiodła się</h1>
<p>Tożsamość lub warunki nie mogły zostać potwierdzone. Spróbuj ponownie lub skontaktuj się z administratorem.</p>
<a class="new-btn" href="/ui/qr">Spróbuj ponownie</a>
"##
)]
struct FailureTemplate;

#[derive(Deserialize)]
struct StatusQuery {
    nonce: String,
}
#[get("/status")]
pub async fn status(Query(StatusQuery { nonce }): Query<StatusQuery>) -> String {
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
