use std::sync::{Arc, Mutex};

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;

use crate::squire;

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
/// * `environment` - Configuration container for the loaded templates.
/// * `request` - A reference to the Actix web `HttpRequest` object.
///
/// # Returns
///
/// Returns an `HttpResponse` with the index page as its body.
#[get("/")]
pub async fn root(environment: web::Data<Arc<Mutex<minijinja::Environment<'static>>>>,
                  request: HttpRequest) -> HttpResponse {
    // Log the connection using the squire::logger::log_connection function.
    squire::logger::log_connection(&request);

    let template = environment.lock().unwrap();
    let index = template.get_template("index").unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(index.render(minijinja::context!()).unwrap())  // no arguments to render
}
