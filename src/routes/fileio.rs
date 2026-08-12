use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use std::path::{Path, PathBuf};
use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use fernet::Fernet;
use serde::Deserialize;

use crate::{constant, routes, squire};
use crate::squire::ffmpeg::JobTracker;

/// Struct to represent the payload data with the URL locator and path locator and the new name for the file.
#[derive(Debug, Deserialize)]
struct Payload {
    url_locator: Option<String>,
    path_locator: Option<String>,
    new_name: Option<String>
}

/// Struct to represent the payload data for an ffmpeg conversion request.
#[derive(Debug, Deserialize)]
struct ConvertPayload {
    url_locator: Option<String>,
    path_locator: Option<String>,
    new_format: Option<String>
}

/// Extracts the relative path from a locator string.
///
/// Locators arrive in two forms: `stream/<path>` (from the listing) or
/// `<host>/stream/<path>` (from the current page URL). A naive `split("stream")`
/// breaks on filenames that themselves contain the word `stream`, so the
/// leading `stream/` prefix is stripped explicitly instead.
///
/// # Arguments
///
/// * `locator` - URL or path locator received from the UI.
///
/// # Returns
///
/// Returns `Some(relative_path)` if the locator is understood, `None` otherwise.
fn extract_relative_path(locator: &str) -> Option<&str> {
    if let Some(rest) = locator.strip_prefix("stream/") {
        return Some(rest);
    }
    locator.split("/stream/").nth(1)
}

/// Extracts the path the file/directory that has to be modified from the payload received.
///
/// # Arguments
///
/// * `url_locator` - URL locator received from the UI as part of the JSON body.
/// * `path_locator` - Path locator received from the UI as part of the JSON body.
/// * `media_source` - Media source configured for the server.
///
/// # Returns
///
/// Returns a result object to describe the status of the extraction.
///
/// * `Ok(PathBuf)` - If the extraction was successful and the path exists in the server.
/// * `Err(String)` - If the extraction has failed or if the path doesn't exist in the server.
fn extract_media_path(url_locator: &str, path_locator: &str, media_source: &Path) -> Result<PathBuf, String> {
    // Create a collection since a tuple is a fixed-size collection in rust and doesn't allow iteration
    for locator in &[url_locator, path_locator] {
        if let Some(relative_path) = extract_relative_path(locator) {
            let path = media_source.join(relative_path);
            if path.exists() {
                log::debug!("Extracted from '{}'", locator);
                return Ok(path);
            }
        }
    }
    Err(String::from("Unable to extract path from either of the parameters"))
}

/// Handles requests for the `/edit` endpoint, to delete/rename media files and directories.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `payload` - JSON payload with `url_path` and `true_path` received from the UI.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
/// * `config` - Configuration data for the application.
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
                  config: web::Data<Arc<squire::settings::Config>>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    let (_host, _last_accessed) = squire::custom::log_connection(&request, &session);
    log::debug!("{}", auth_response.detail);
    let extracted = match (payload.url_locator.as_deref(), payload.path_locator.as_deref()) {
        (Some(url), Some(path)) => extract_media_path(url, path, &config.media_source),
        _ => Err(String::from("Both URL locator and path locator must be provided"))
    };
    // todo: styling of the pop up is very basic
    let media_path: PathBuf = match extracted {
        Ok(path) => {
            path
        },
        Err(msg) => {
            return HttpResponse::BadRequest().body(msg);
        }
    };
    if let Some(edit_action) = request.headers().get("edit-action") {
        let action = edit_action.to_str().unwrap();
        log::info!("{} requested to {} {:?}", auth_response.username, action, media_path);
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
            log::warn!("Unsupported action: {} requested to {} {:?}", auth_response.username, action, media_path);
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
/// - `Condition 3` - Validate if the new filename has at least one character, apart from the file extension.
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
    let new_extension = new_name.split('.').next_back().unwrap_or_default();
    if new_name.len() <= new_extension.len() + 1 {
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
    match validity {
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
    }
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

/// Handles requests for the `/convert` endpoint, starting an ffmpeg conversion.
///
/// The conversion runs in a background thread, the endpoint responds
/// immediately with the job id. ffmpeg is spawned only for this job and is
/// terminated once the conversion finishes. The converted file is created next
/// to the original file and the original file is never modified or removed.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `payload` - JSON payload with `url_locator`, `path_locator` and `new_format` received from the UI.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
/// * `config` - Configuration data for the application.
/// * `jobs` - Tracker for the background conversion jobs.
///
/// # Returns
///
/// * `202` - HttpResponse with the job id for status polling.
/// * `400` - HttpResponse with an error message for an invalid request.
/// * `403` - HttpResponse with an error message when ffmpeg support is disabled.
/// * `409` - HttpResponse when a conversion to the same target file is already running.
#[allow(clippy::too_many_arguments)]
#[post("/convert")]
pub async fn convert(request: HttpRequest,
                     payload: web::Json<ConvertPayload>,
                     fernet: web::Data<Arc<Fernet>>,
                     session: web::Data<Arc<constant::Session>>,
                     config: web::Data<Arc<squire::settings::Config>>,
                     jobs: web::Data<Arc<JobTracker>>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    if !config.ffmpeg_enabled {
        let reason = "ffmpeg conversion is disabled on this server";
        log::warn!("{} requested a conversion, but {}", auth_response.username, reason);
        return HttpResponse::Forbidden().body(reason);
    }
    let (_host, _last_accessed) = squire::custom::log_connection(&request, &session);
    log::debug!("{}", auth_response.detail);
    let media_path = match (payload.url_locator.as_deref(), payload.path_locator.as_deref()) {
        (Some(url), Some(path)) => match extract_media_path(url, path, &config.media_source) {
            Ok(path) => path,
            Err(msg) => return HttpResponse::BadRequest().body(msg)
        },
        _ => return HttpResponse::BadRequest().body("Both URL locator and path locator must be provided")
    };
    let target_format = match payload.new_format.as_deref() {
        Some(format) => format.trim(),
        None => return HttpResponse::BadRequest().body("New format is missing!")
    };
    if target_format.is_empty() {
        return HttpResponse::BadRequest().body("New format is missing!");
    }
    // Fail fast on invalid requests, only the ffmpeg execution runs in the background
    let output_path = match squire::ffmpeg::validate(&media_path, target_format) {
        Ok(path) => path,
        Err(reason) => return HttpResponse::BadRequest().body(reason),
    };
    if jobs.is_running(&output_path) {
        let reason = "A conversion to this target file is already in progress";
        log::warn!("{} requested a conversion, but {}", auth_response.username, reason);
        return HttpResponse::Conflict().body(reason);
    }
    let file_name = media_path.file_name().unwrap().to_string_lossy().to_string();
    let job_id = format!(
        "{}_{}",
        file_name,
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    );
    jobs.insert(&job_id, &output_path.to_string_lossy());
    log::info!("{} requested to convert {:?} to '{}' [job {}]", auth_response.username, media_path, target_format, job_id);
    let job_tracker = jobs.clone();
    let convert_path = media_path.clone();
    let convert_format = target_format.to_string();
    let job_id_clone = job_id.clone();
    std::thread::spawn(move || {
        match squire::ffmpeg::convert(&convert_path, &convert_format) {
            Ok(output) => job_tracker.finish(&job_id_clone, "done", output),
            Err(reason) => job_tracker.finish(&job_id_clone, "failed", reason),
        }
    });
    HttpResponse::Accepted().json(serde_json::json!({ "job": job_id }))
}

/// Handles requests for the `/convert/status/{job_id}` endpoint, returning the state of a conversion job.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `job_id` - Job id received in the URL path.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
/// * `config` - Configuration data for the application.
/// * `jobs` - Tracker for the background conversion jobs.
///
/// # Returns
///
/// * `200` - HttpResponse with the job state (`running`/`done`/`failed`) and detail.
/// * `404` - HttpResponse when the job id is unknown.
#[get("/convert/status/{job_id:.*}")]
pub async fn convert_status(request: HttpRequest,
                            job_id: web::Path<String>,
                            fernet: web::Data<Arc<Fernet>>,
                            session: web::Data<Arc<constant::Session>>,
                            config: web::Data<Arc<squire::settings::Config>>,
                            jobs: web::Data<Arc<JobTracker>>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    match jobs.get(&job_id) {
        Some(job) => HttpResponse::Ok().json(job),
        None => HttpResponse::NotFound().body("Unknown job")
    }
}
