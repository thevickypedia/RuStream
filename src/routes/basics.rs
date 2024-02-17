use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::StatusCode;

use crate::{squire, jinja};

/// Handles the health endpoint, returning a JSON response indicating the server is healthy.
///
/// # Returns
///
/// Returns an `HttpResponse` with a status of 200 (OK), content type "application/json",
/// and a JSON body containing the string "Healthy".
#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json("Healthy")
}

/// Handles the root endpoint, logging the connection and returning an HTML response.
///
/// # Arguments
///
/// * `request` - The HTTP request received for the root endpoint.
///
/// # Returns
///
/// Returns an `HttpResponse` with the index page as its body.
#[get("/")]
pub async fn root(request: HttpRequest) -> HttpResponse {
    // Log the connection using the squire::logger::log_connection function.
    squire::logger::log_connection(&request);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(jinja::get_content("index"))
}
