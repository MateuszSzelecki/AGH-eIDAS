use actix_web::get;
use askama::Template;

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
<h1>🔐 Weryfikator eIDAS</h1>
<p class="subtitle">Zero-Knowledge Proof Verification</p>

<div id="challenge"></div>

<button class="refresh-btn" hx-get="/ui/challenge" hx-target="#challenge" hx-swap="outerHTML">
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
