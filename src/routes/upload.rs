use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use actix_multipart::Multipart;
use actix_web::{http, HttpRequest, HttpResponse, web};
use fernet::Fernet;
use futures_util::StreamExt as _;

use crate::{constant, routes, squire};

/// Saves files locally by breaking them into chunks.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `payload` - Mutable multipart struct that is sent from the UI as `FormData`.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
/// * `config` - Configuration data for the application.
///
/// ## References
/// - [Server Side](https://docs.rs/actix-multipart/latest/actix_multipart/struct.Multipart.html)
/// - [Client Side (not implemented)](https://accreditly.io/articles/uploading-large-files-with-chunking-in-javascript)
///
/// # Returns
///
/// * `200` - Plain HTTPResponse indicating that the file was uploaded.
/// * `422` - HTTPResponse with JSON object indicating that the payload was incomplete.
/// * `400` - HTTPResponse with JSON object indicating that the payload was invalid.
#[post("/upload")]
pub async fn save_files(request: HttpRequest,
                        mut payload: Multipart,
                        fernet: web::Data<Arc<Fernet>>,
                        session: web::Data<Arc<constant::Session>>,
                        config: web::Data<Arc<squire::settings::Config>>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    let mut upload_path = config.media_source.clone();  // cannot be borrowed as mutable
    let mut secure_str = "";
    if let Some(dedicated) = request.headers().get("dedicated_directory") {
        if dedicated.to_str().unwrap_or("false") == "true" {
            secure_str = "to secure index ";
            upload_path.extend([format!("{}_{}", &auth_response.username, constant::SECURE_INDEX)])
        }
    }
    while let Some(item) = payload.next().await {
        match item {
            Ok(mut field) => {
                let filename = field.content_disposition().get_filename().unwrap();
                let mut destination = File::create(&upload_path.join(filename)).unwrap();
                log::info!("Downloading '{}' {}- uploaded by '{}'", &filename, secure_str, &auth_response.username);
                while let Some(fragment) = field.next().await {
                    match fragment {
                        Ok(chunk) => {
                            destination.write_all(&chunk).unwrap();
                        }
                        Err(err) => {
                            // User might have aborted file upload
                            let error = format!("Error processing chunk: {}", err);
                            log::warn!("{}", &error);
                            return HttpResponse::UnprocessableEntity().json(error);
                        }
                    }
                }
            }
            Err(err) => {
                let error = format!("Error processing field: {}", err);
                log::error!("{}", &error);
                return HttpResponse::BadRequest().json(error);
            }
        }
    }
    HttpResponse::Ok().finish()
}

/// Handles requests for the `/upload` endpoint, serving the file upload template.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `config` - Configuration data for the application.
/// * `template` - Configuration container for the loaded templates.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
///
/// # Returns
///
/// Returns an `HttpResponse` with the upload page as its body.
#[get("/upload")]
pub async fn upload_files(request: HttpRequest,
                          fernet: web::Data<Arc<Fernet>>,
                          config: web::Data<Arc<squire::settings::Config>>,
                          template: web::Data<Arc<minijinja::Environment<'static>>>,
                          session: web::Data<Arc<constant::Session>>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    let landing = template.get_template("upload").unwrap();
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(landing.render(minijinja::context!(
            user => auth_response.username,
            secure_index => constant::SECURE_INDEX
        )).unwrap())
}
