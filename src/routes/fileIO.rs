use std::fs::{remove_dir_all, remove_file};

use std::path::{Path, PathBuf};
use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use fernet::Fernet;
use serde::Deserialize;

use crate::{constant, routes, squire};

/// Struct to represent the payload data with both the URL locator and path locator
#[derive(Debug, Deserialize)]
struct Payload {
    url_locator: Option<String>,
    path_locator: Option<String>,
}

/// Extracts the path the file/directory that has to be deleted from the payload received.
///
/// # Arguments
///
/// * `payload` - Payload received from the UI as JSON body.
/// * `media_source` - Media source configured for the server.
///
/// # Returns
///
/// Returns a result object to describe the status of the extraction.
///
/// * `Ok(PathBuf)` - If the extraction was successful and the path exists in the server.
/// * `Err(String)` - If the extraction has failed or if the path doesn't exist in the server.
fn extract_media_path(payload: web::Json<Payload>, media_source: &Path) -> Result<PathBuf, String> {
    let url_locator = payload.url_locator.as_deref();
    let path_locator = payload.path_locator.as_deref();
    if let (Some(url_str), Some(path_str)) = (url_locator, path_locator) {
        // Create a collection since a tuple is a fixed-size collection in rust and doesn't allow iteration
        for locator in &[url_str, path_str] {
            if let Some(media_path) = locator.split("stream").nth(1) {
                // Without stripping the '/' in front of the path, Rust will assume that's a root path
                // This will overwrite media_source and render the joined path instead of combining the two
                let path = media_source.join(media_path.strip_prefix('/').unwrap());
                if path.exists() {
                    log::debug!("Extracted from '{}'", locator);
                    return Ok(path);
                }
            }
        }
        return Err(String::from("Unable to extract path from either of the parameters"));
    }
    Err(String::from("Both URL locator and path locator must be provided"))
}

/// Handles requests for the `/edit` endpoint, to delete/rename media files and directories.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `payload` - JSON payload with `url_path` and `true_path` received from the UI.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
/// * `metadata` - Struct containing metadata of the application.
/// * `config` - Configuration data for the application.
/// * `template` - Configuration container for the loaded templates.
///
/// # Returns
///
/// * `200` - HttpResponse with a `session_token` and redirect URL to the `/home` entrypoint.
/// * `400` - HttpResponse with an error message for invalid action or incorrect payload.
/// * `401` - HttpResponse with an error message for failed authentication.
#[post("/edit")]
pub async fn edit(request: HttpRequest,
                  payload: web::Json<Payload>,
                  fernet: web::Data<Arc<Fernet>>,
                  session: web::Data<Arc<constant::Session>>,
                  metadata: web::Data<Arc<constant::MetaData>>,
                  config: web::Data<Arc<squire::settings::Config>>,
                  template: web::Data<Arc<minijinja::Environment<'static>>>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    let (_host, _last_accessed) = squire::logger::log_connection(&request, &session);
    log::debug!("{}", auth_response.detail);
    let extracted = extract_media_path(payload, &config.media_source);
    // todo: pop up doesn't always occur next to the mouse
    //  styling of the pop up is very basic
    //  make custom error responses generic
    let media_path: PathBuf = match extracted {
        Ok(path) => {
            path
        },
        Err(msg) => {
            return HttpResponse::BadRequest().body(msg);
        }
    };
    if !squire::authenticator::verify_secure_index(&PathBuf::from(&media_path), &auth_response.username) {
        return squire::responses::restricted(
            template.get_template("error").unwrap(),
            &auth_response.username,
            &metadata.pkg_version,
        );
    }
    if let Some(edit_action) = request.headers().get("edit-action") {
        let action = edit_action.to_str().unwrap();
        return if action == "delete" {
            log::info!("{} requested to delete {:?}", &auth_response.username, &media_path);
            if media_path.is_file() {
                if let Err(error) = remove_file(media_path) {
                    let reason = format!("Error deleting file: {}", error);
                    HttpResponse::InternalServerError().body(reason)
                } else {
                    HttpResponse::Ok().finish()
                }
            } else if media_path.is_dir() {
                if let Err(error) = remove_dir_all(media_path) {
                    let reason = format!("Error deleting directory: {}", error);
                    HttpResponse::InternalServerError().body(reason)
                } else {
                    HttpResponse::Ok().finish()
                }
            } else {
                let reason = format!("{:?} was neither a file nor a directory", media_path);
                log::warn!("{}", reason);
                HttpResponse::BadRequest().body(reason)
            }
        } else {
            log::warn!("Unsupported action: {} requested to {} {:?}", &auth_response.username, action, &media_path);
            HttpResponse::BadRequest().body("Unsupported action!")
        };
    }
    log::warn!("No action received for: {:?}", media_path);
    HttpResponse::BadRequest().body("No action received!")
}
