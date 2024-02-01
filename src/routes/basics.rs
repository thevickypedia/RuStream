use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use crate::render;

#[get("/health")]
pub async fn health() -> HttpResponse {
    return HttpResponse::Ok().content_type("application/json").json("Healthy");
}

#[get("/status")]
pub async fn status() -> HttpResponse {
    return HttpResponse::Ok().content_type("application/json").json("Status");
}

#[get("/")]
pub async fn root() -> HttpResponse {
    return HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(render::INDEX);
}
