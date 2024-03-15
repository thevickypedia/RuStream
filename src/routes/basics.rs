use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use fernet::Fernet;

use crate::{constant, routes, squire};

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
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
/// * `metadata` - Struct containing metadata of the application.
/// * `template` - Configuration container for the loaded templates.
///
/// # Returns
///
/// Returns an `HttpResponse` with the index page as its body.
#[get("/")]
pub async fn root(request: HttpRequest,
                  session: web::Data<Arc<constant::Session>>,
                  metadata: web::Data<Arc<constant::MetaData>>,
                  template: web::Data<Arc<minijinja::Environment<'static>>>) -> HttpResponse {
    let (_host, _last_accessed) = squire::logger::log_connection(&request, &session);
    let index = template.get_template("index").unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(index.render(minijinja::context!(version => &metadata.pkg_version)).unwrap())
}

/// Handles the profile endpoint, and returns an HTML response.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
/// * `metadata` - Struct containing metadata of the application.
/// * `config` - Configuration data for the application.
/// * `template` - Configuration container for the loaded templates.
///
/// # Returns
///
/// Returns an `HttpResponse` with the profile page as its body.
#[get("/profile")]
pub async fn profile(request: HttpRequest,
                     fernet: web::Data<Arc<Fernet>>,
                     session: web::Data<Arc<constant::Session>>,
                     metadata: web::Data<Arc<constant::MetaData>>,
                     config: web::Data<Arc<squire::settings::Config>>,
                     template: web::Data<Arc<minijinja::Environment<'static>>>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    let (_host, last_accessed) = squire::logger::log_connection(&request, &session);
    let index = template.get_template("profile").unwrap();
    let mut access_map = HashMap::new();
    if !last_accessed.is_empty() {
        let filepath = Path::new(&last_accessed);
        let extn = filepath.extension().unwrap().to_str().unwrap();
        let name = filepath.iter().last().unwrap().to_string_lossy().to_string();
        let path = format!("/stream/{}", &last_accessed);
        let font = if last_accessed.contains(constant::SECURE_INDEX) {
            "fa-solid fa-lock".to_string()
        } else {
            squire::content::get_file_font(extn)
        };
        access_map = HashMap::from([
            ("name", name), ("font", font), ("path", path)
        ]);
    }
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(index.render(minijinja::context!(
            version => &metadata.pkg_version,
            user => &auth_response.username,
            time_left => &auth_response.time_left,
            file => access_map,
        )).unwrap())
}
