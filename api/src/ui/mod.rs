mod challenge;
mod status;
mod verification;

use actix_web::{HttpResponse, get, web};
use askama::Template;

#[derive(Template)]
#[template(
    ext = "html",
    source = r##"
{% extends "base.html" %}

{% block content %}
<div hx-get="/ui/verification" hx-trigger="load" hx-target="#container" hx-swap="innerHTML"></div>
{% endblock %}
"##
)]
struct IndexTemplate;

#[get("")]
async fn index() -> HttpResponse {
    IndexTemplate
        .render()
        .map(|template| HttpResponse::Ok().body(template))
        .unwrap_or(HttpResponse::InternalServerError().body("Internal Server Error"))
}

pub fn scope() -> actix_web::Scope {
    web::scope("/ui")
        .service(index)
        .service(verification::verification)
        .service(challenge::challenge)
        .service(status::status)
}
