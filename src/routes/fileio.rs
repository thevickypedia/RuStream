use std::fs;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use fernet::Fernet;
use serde::Deserialize;

use crate::{constant, routes, squire};

/// Struct to represent the payload data with the URL locator and path locator and the new name for the file.
#[derive(Debug, Deserialize)]
struct Payload {
    url_locator: Option<String>,
    path_locator: Option<String>,
    new_name: Option<String>
}

/// Extracts the path the file/directory that has to be modified from the payload received.
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
fn extract_media_path(payload: &web::Json<Payload>, media_source: &Path) -> Result<PathBuf, String> {
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
/// * `200` - Blank HttpResponse to indicate that the request was successful.
/// * `400` - HttpResponse with an error message for invalid action or incorrect payload.
/// * `401` - HttpResponse with an error message for failed authentication.
/// * `500` - HttpResponse with an error message for failed delete/rename.
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
    let (_host, _last_accessed) = squire::custom::log_connection(&request, &session);
    log::debug!("{}", auth_response.detail);
    let extracted = extract_media_path(&payload, &config.media_source);
    // todo: styling of the pop up is very basic
    let media_path: PathBuf = match extracted {
        Ok(path) => {
            path
        },
        Err(msg) => {
            return HttpResponse::BadRequest().body(msg);
        }
    };
    if !squire::authenticator::verify_secure_index(&PathBuf::from(&media_path), &auth_response.username) {
        return squire::custom::error(
            "RESTRICTED SECTION",
            template.get_template("error").unwrap(),
            &metadata.pkg_version,
            format!("This content is not accessible, as it does not belong to the user profile '{}'", auth_response.username),
            StatusCode::FORBIDDEN
        );
    }
    if let Some(edit_action) = request.headers().get("edit-action") {
        let action = edit_action.to_str().unwrap();
        log::info!("{} requested to {} {:?}", &auth_response.username, action, &media_path);
        return if action == "delete" {
            return delete(media_path);
        } else if action == "rename" {
            let new_name_str = payload.new_name.as_deref();
            if let Some(new_name) = new_name_str {
                return rename(media_path, new_name.trim());
            } else {
                HttpResponse::BadRequest().body("New name is missing!")
            }
        } else {
            log::warn!("Unsupported action: {} requested to {} {:?}", &auth_response.username, action, &media_path);
            HttpResponse::BadRequest().body("Unsupported action!")
        };
    }
    log::warn!("No action received for: {:?}", media_path);
    HttpResponse::BadRequest().body("No action received!")
}

/// Checks if the new filename is valid with multiple conditions.
///
/// # Arguments
///
/// * `old_filepath` - PathBuf object to the file that has to be renamed.
/// * `new_name` - New name for the file.
///
/// ## See Also
///
/// - `Condition 1` - Validate if the new filename is the same as old.
/// - `Condition 2` - Validate if the new filename starts or ends with `.` or `_`
/// - `Condition 3` - Validate if the new filename and the old has the same file extension.
/// - `Condition 4` - Validate if the new filename has at least one character, apart from the file extension.
///
/// # Returns
///
/// Returns a result object to describe the status of the validation.
///
/// * `Ok(bool)` - If the new name has passed all the validations.
/// * `Err(String)` - If the validation has failed.
fn is_valid_name(old_filepath: &PathBuf, new_name: &str) -> Result<bool, String> {
    let old_name_str = old_filepath.file_name().unwrap_or_default().to_str().unwrap_or_default();
    if old_name_str == new_name {
        return Err(format!("New name cannot be the same as old\n\n'{:?}'=='{new_name}'", old_filepath))
    }
    if new_name.starts_with('_') || new_name.ends_with('_') ||
        new_name.starts_with('.') || new_name.ends_with('.') {
        return Err(format!("New name cannot start or end with '.' or '_'\n\n'{}'", new_name))
    }
    let old_extension = old_filepath.extension().unwrap().to_str().unwrap();
    let new_extension = new_name.split('.').last().unwrap_or_default();
    if old_extension != new_extension {
        return Err(format!("File extension cannot be changed\n\n'{new_extension}' => '{old_extension}'"))
    }
    if new_name.len() <= old_extension.len() + 1 {
        return Err(format!("At least one character is required as filename\n\nReceived {}", new_name.len()))
    }
    Ok(true)
}

/// Renames the file.
///
/// # Arguments
///
/// - `old_filepath` - PathBuf object to the file that has to be renamed.
/// - `new_name` - New name for the file.
///
/// # Returns
///
/// * `200` - Blank HttpResponse to indicate that the request was successful.
/// * `400` - HttpResponse with an error message for invalid action or incorrect payload.
/// * `500` - HttpResponse with an error message for failed rename.
fn rename(media_path: PathBuf, new_name: &str) -> HttpResponse {
    if new_name.is_empty() {
        let reason = "New name not received in payload";
        log::warn!("{}", reason);
        return HttpResponse::BadRequest().body(reason);
    }
    if !media_path.is_file() {
        let reason = format!("{:?} is an invalid file entry", media_path);
        return HttpResponse::BadRequest().body(reason);
    }
    let validity = is_valid_name(
        &media_path, new_name
    );
    return match validity {
        Ok(_) => {
            let new_path = media_path.parent().unwrap().join(new_name).to_string_lossy().to_string();
            let old_path = media_path.to_string_lossy().to_string();
            if let Err(error) = fs::rename(old_path, new_path) {
                let reason = format!("Error renaming file: {}", error);
                log::error!("{}", reason);
                HttpResponse::InternalServerError().body(reason)
            } else {
                HttpResponse::Ok().finish()
            }
        },
        Err(msg) => {
            HttpResponse::BadRequest().body(msg)
        }
    };
}

/// Deletes the file.
///
/// # Arguments
///
/// - `media_path` - PathBuf object to the file that has to be deleted.
///
/// # Returns
///
/// * `200` - Blank HttpResponse to indicate that the request was successful.
/// * `400` - HttpResponse with an error message for invalid action or incorrect payload.
/// * `500` - HttpResponse with an error message for failed delete.
fn delete(media_path: PathBuf) -> HttpResponse {
    if media_path.is_file() {
        if let Err(error) = fs::remove_file(media_path) {
            let reason = format!("Error deleting file: {}", error);
            log::error!("{}", reason);
            HttpResponse::InternalServerError().body(reason)
        } else {
            HttpResponse::Ok().finish()
        }
    } else if media_path.is_dir() {
        if let Err(error) = fs::remove_dir_all(media_path) {
            let reason = format!("Error deleting directory: {}", error);
            log::error!("{}", reason);
            HttpResponse::InternalServerError().body(reason)
        } else {
            HttpResponse::Ok().finish()
        }
    } else {
        let reason = format!("{:?} was neither a file nor a directory", media_path);
        log::warn!("{}", reason);
        HttpResponse::BadRequest().body(reason)
    }
}
