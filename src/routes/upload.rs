use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

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
/// * `401` - HTTPResponse with JSON payload indicating the problem uploading file.
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
    let source_path = Path::new("uploads");
    while let Some(item) = payload.next().await {
        match item {
            Ok(mut field) => {
                let filename = field.content_disposition().get_filename().unwrap();
                let mut destination = File::create(source_path.join(filename)).unwrap();
                let start = Instant::now();
                log::info!("Downloading '{}'", &filename);
                while let Some(fragment) = field.next().await {
                    match fragment {
                        Ok(chunk) => {
                            destination.write_all(&chunk).unwrap();
                        }
                        Err(err) => {
                            let error = format!("Error processing field: {}", err);
                            log::error!("{}", &error);
                            return HttpResponse::InternalServerError().json(error);
                        }
                    }
                }
                // todo: Remove this or set to debug
                log::info!("Download completed in {}s", start.elapsed().as_secs())
            }
            Err(err) => {
                let error = format!("Error processing field: {}", err);
                log::error!("{}", &error);
                return HttpResponse::InternalServerError().json(error);
            }
        }
    }
    HttpResponse::Ok().finish()
}

/// Handles requests for the '/upload' endpoint, serving the file upload template.
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
    // todo: Create user specific directories and upload there instead
    let upload_path = Path::new("uploads");
    if !upload_path.exists() {
        match std::fs::create_dir("uploads") {
            Ok(_) => {
                log::info!("Created uploads directory successfully");
            }
            Err(err) => {
                log::error!("{}", err);
            }
        }
    }
    let landing = template.get_template("upload").unwrap();
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(landing.render(minijinja::context!()).unwrap())  // no arguments to render
}
