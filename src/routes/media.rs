use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use fernet::Fernet;
use minijinja;
use serde::Deserialize;
use url::form_urlencoded;


use crate::{constant, routes, squire};

/// Represents the payload structure for deserializing data from the request query parameters.
#[derive(Deserialize)]
pub struct Payload {
    file: String,
}

/// Represents the paths and filenames for subtitles, including both SRT and VTT formats.
struct Subtitles {
    srt: PathBuf,
    vtt: PathBuf,
    vtt_file: String,
}

/// URL encodes the provided path string.
///
/// This function takes a reference to a `String` representing a path,
/// encodes it using the `form_urlencoded` crate, and returns the encoded string.
///
/// # Arguments
///
/// * `path` - The input path string to be URL encoded.
///
/// ## References
/// - [rustjobs.dev](https://rustjobs.dev/blog/how-to-url-encode-strings-in-rust/)
///
/// # Returns
///
/// Returns a URL encoded string.
fn url_encode(path: &String) -> String {
    form_urlencoded::byte_serialize(path.as_bytes())
        .collect::<Vec<_>>()
        .join("")
}

/// Constructs a `Subtitles` struct based on the provided `target` path and `target_str`.
///
/// # Arguments
///
/// * `true_path` - True path of the requested file.
/// * `relative_path` - The string representation of the relative filepath.
///
/// # Returns
///
/// Returns a `Subtitles` struct containing paths and filenames for both SRT and VTT subtitle files.
fn subtitles(true_path: PathBuf, relative_path: &String) -> Subtitles {
    // Set srt and vtt extensions to true path to check if they exist
    let srt = true_path.with_extension("srt");
    let vtt = true_path.with_extension("vtt");

    // Set vtt extension to the relative path, so it could be used as a parameter in HTML
    let vtt_filepath = PathBuf::new().join(relative_path).with_extension("vtt");
    let vtt_file = vtt_filepath.to_string_lossy().to_string();

    Subtitles { srt, vtt, vtt_file }
}

/// Handles requests for the '/track/{track_path:.*}' endpoint, serving track files.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `info` - Query string from the request.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
/// * `metadata` - Struct containing metadata of the application.
/// * `config` - Configuration data for the application.
/// * `template` - Configuration container for the loaded templates.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the track file content or an error response.
#[get("/track")]
pub async fn track(request: HttpRequest,
                   info: web::Query<Payload>,
                   fernet: web::Data<Arc<Fernet>>,
                   session: web::Data<Arc<constant::Session>>,
                   metadata: web::Data<Arc<constant::MetaData>>,
                   config: web::Data<Arc<squire::settings::Config>>,
                   template: web::Data<Arc<minijinja::Environment<'static>>>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    if !squire::authenticator::verify_secure_index(&PathBuf::from(&info.file), &auth_response.username) {
        return squire::custom::error(
            "RESTRICTED SECTION",
            template.get_template("error").unwrap(),
            &metadata.pkg_version,
            format!("This content is not accessible, as it does not belong to the user profile '{}'", auth_response.username),
            StatusCode::FORBIDDEN
        );
    }
    let (_host, _last_accessed) = squire::custom::log_connection(&request, &session);
    log::debug!("{}", auth_response.detail);
    log::debug!("Track requested: {}", &info.file);
    let filepath = Path::new(&config.media_source).join(&info.file);
    log::debug!("Track file lookup: {}", &filepath.to_string_lossy());
    match std::fs::read_to_string(&filepath) {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/plain")
            .body(content),
        Err(_) => squire::custom::error(
            "CONTENT UNAVAILABLE",
            template.get_template("error").unwrap(),
            &metadata.pkg_version,
            format!("'{}' was not found", &info.file),
            StatusCode::NOT_FOUND
        )
    }
}

/// Create an `HttpResponse` based on the context built and rendered template.
///
/// # Arguments
///
/// * `landing` - `Template` retrieved from the configuration container.
/// * `serializable` - `HashMap` that can be serialized into a single block of String to be rendered.
fn render_content(landing: minijinja::Template,
                  serializable: HashMap<&str, &String>) -> HttpResponse {
    return match landing.render(serializable) {
        Ok(response_body) => {
            HttpResponse::build(StatusCode::OK)
                .content_type("text/html; charset=utf-8").body(response_body)
        }
        Err(err) => {
            log::error!("{}", err);
            HttpResponse::FailedDependency().json("Failed to render content.")
        }
    };
}

/// Handles requests for the `/stream/{media_path:.*}` endpoint, serving media files and directories.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `media_path` - The path parameter representing the media file or directory.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
/// * `metadata` - Struct containing metadata of the application.
/// * `config` - Configuration data for the application.
/// * `template` - Configuration container for the loaded templates.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the media content or directory listing, or an error response.
#[get("/stream/{media_path:.*}")]
pub async fn stream(request: HttpRequest,
                    media_path: web::Path<String>,
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
    let filepath = media_path.to_string();
    if !squire::authenticator::verify_secure_index(&PathBuf::from(&filepath), &auth_response.username) {
        return squire::custom::error(
            "RESTRICTED SECTION",
            template.get_template("error").unwrap(),
            &metadata.pkg_version,
            format!("This content is not accessible, as it does not belong to the user profile '{}'", auth_response.username),
            StatusCode::FORBIDDEN
        );
    }
    let secure_path = if filepath.contains(constant::SECURE_INDEX) { "true" } else { "false" };
    let secure_flag = secure_path.to_string();
    // True path of the media file
    let __target = config.media_source.join(&filepath);
    if !__target.exists() {
        return squire::custom::error(
            "CONTENT UNAVAILABLE",
            template.get_template("error").unwrap(),
            &metadata.pkg_version,
            format!("'{}' was not found", filepath),
            StatusCode::NOT_FOUND
        )
    }
    // True path of the media file as a String
    let __target_str = __target.to_string_lossy().to_string();
    let __filename = __target.file_name().unwrap().to_string_lossy().to_string();
    if __target.is_file() {
        let landing = template.get_template("landing").unwrap();
        let rust_iter = squire::content::get_iter(&__target, &config.file_formats);
        let render_path = format!("/media?file={}", url_encode(&filepath));
        let prev = rust_iter.previous.unwrap_or_default();
        let next = rust_iter.next.unwrap_or_default();
        let secure_index = constant::SECURE_INDEX.to_string();
        let mut context_builder = vec![
            ("version", &metadata.pkg_version),
            ("media_title", &__filename),
            ("path", &render_path),
            ("previous", &prev),
            ("next", &next),
            ("user", &auth_response.username),
            ("secure_index", &secure_index),
        ].into_iter().collect::<HashMap<_, _>>();
        if constant::IMAGE_FORMATS
            .contains(&render_path.split('.')
                .last()
                .unwrap()  // file extension WILL be present at this point
                .to_lowercase().as_str()) {
            context_builder.insert("render_image", &render_path);
            return render_content(landing, context_builder);
        }
        let subtitle = subtitles(__target, &filepath);
        let mut sfx_file = String::new();
        if subtitle.vtt.exists() {
            sfx_file = format!("/track?file={}", url_encode(&subtitle.vtt_file));
        } else if subtitle.srt.exists() {
            log::info!("Converting {:?} to {:?} for subtitles",
                subtitle.srt.file_name().unwrap(),
                subtitle.vtt.file_name().unwrap());
            match squire::subtitles::srt_to_vtt(&subtitle.srt) {
                Ok(_) => {
                    log::debug!("Successfully converted srt to vtt file");
                    sfx_file = format!("/track?file={}", url_encode(&subtitle.vtt_file));
                }
                Err(err) => log::error!("Failed to convert srt to vtt: {}", err),
            }
        }
        if !sfx_file.is_empty() {
            context_builder.insert("track", &sfx_file);
        }
        return render_content(landing, context_builder);
    } else if __target.is_dir() {
        let child_dir = __target.iter().last().unwrap().to_string_lossy().to_string();
        let listing_page = squire::content::get_dir_stream_content(&__target_str, &child_dir, &config.file_formats);
        let listing = template.get_template("listing").unwrap();
        let custom_title = if child_dir.ends_with(constant::SECURE_INDEX) {
            format!(
                "<i class='fa-solid fa-lock'></i>&nbsp;&nbsp;{}",
                child_dir.strip_suffix(&format!("_{}", constant::SECURE_INDEX)).unwrap()
            )
        } else {
            child_dir
        };
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(listing.render(minijinja::context!(
                version => metadata.pkg_version,
                custom_title => custom_title,
                files => listing_page.files,
                user => auth_response.username,
                secure_index => constant::SECURE_INDEX,
                directories => listing_page.directories,
                secured_directories => listing_page.secured_directories,
                secure_path => &secure_flag
            )).unwrap());
    }
    log::error!("Something went horribly wrong");
    log::error!("Media Path: {}", filepath);
    log::error!("Target: {}", __target_str);
    HttpResponse::ExpectationFailed().json(routes::auth::DetailError {
        detail: format!("'{}' was neither a file nor a folder", filepath)
    })
}

/// Handles requests for the `/media` endpoint, serving media content for streaming.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `info` - The query parameter containing the file information.
/// * `fernet` - Fernet object to encrypt the auth payload that will be set as `session_token` cookie.
/// * `session` - Session struct that holds the `session_mapping` and `session_tracker` to handle sessions.
/// * `metadata` - Struct containing metadata of the application.
/// * `config` - Configuration data for the application.
/// * `template` - Configuration container for the loaded templates.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the media content or an error response.
#[get("/media")]
pub async fn streaming_endpoint(request: HttpRequest,
                                info: web::Query<Payload>,
                                fernet: web::Data<Arc<Fernet>>,
                                session: web::Data<Arc<constant::Session>>,
                                metadata: web::Data<Arc<constant::MetaData>>,
                                config: web::Data<Arc<squire::settings::Config>>,
                                template: web::Data<Arc<minijinja::Environment<'static>>>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    let media_path = config.media_source.join(&info.file);
    if !squire::authenticator::verify_secure_index(&media_path, &auth_response.username) {
        return squire::custom::error(
            "RESTRICTED SECTION",
            template.get_template("error").unwrap(),
            &metadata.pkg_version,
            format!("This content is not accessible, as it does not belong to the user profile '{}'", auth_response.username),
            StatusCode::FORBIDDEN
        );
    }
    let (host, _last_accessed) = squire::custom::log_connection(&request, &session);
    if media_path.exists() {
        let file = actix_files::NamedFile::open_async(media_path).await.unwrap();
        // Check if the host is making a continued connection streaming the same file
        let mut tracker = session.tracker.lock().unwrap();
        if tracker.get(&host).unwrap() != &info.file {
            log::info!("Streaming {}", info.file);
            tracker.insert(host, info.file.to_string());
        }
        return file.into_response(&request);
    }
    let error = format!("File {:?} not found", media_path);
    log::error!("{}", error);
    squire::custom::error(
        "CONTENT UNAVAILABLE",
        template.get_template("error").unwrap(),
        &metadata.pkg_version,
        format!("'{}' was not found", &info.file),
        StatusCode::NOT_FOUND
    )
}
