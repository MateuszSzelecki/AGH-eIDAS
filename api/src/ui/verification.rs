use actix_web::get;
use askama::Template;

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
<style>
    h1 {
        color: #667eea;
        margin-bottom: 10px;
    }
    .subtitle {
        color: #666;
        margin-bottom: 20px;
    }
    .refresh-btn {
        margin-top: 20px;
        padding: 12px 24px;
        background-color: #667eea;
        color: white;
        border: none;
        border-radius: 6px;
        cursor: pointer;
        font-size: 1em;
        transition: background-color 0.3s;
    }
    .refresh-btn:hover {
        background-color: #5568d3;
    }
</style>

<h1>🔐 Weryfikator eIDAS</h1>
<p class="subtitle">Zero-Knowledge Proof Verification</p>

<div id="challenge" hx-get="/ui/challenge" hx-trigger="load" hx-target="#challenge" hx-swap="innerHTML"></div>

<button class="refresh-btn" hx-get="/ui/challenge" hx-target="#challenge" hx-swap="innerHTML">
    🔄 Wygeneruj nowy Challenge
</button>
"##
)]
struct VerificationTemplate;

#[get("/verification")]
pub async fn verification() -> String {
    VerificationTemplate
        .render()
        .unwrap_or("Failed to start the verification process".to_string())
}
