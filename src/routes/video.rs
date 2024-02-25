use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use minijinja::{context, Environment, Template};
use serde::Deserialize;
use url::form_urlencoded;

use crate::{constant, squire};
use crate::routes;

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
/// - [RustJobs](https://rustjobs.dev/blog/how-to-url-encode-strings-in-rust/)
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
/// * `true_path` - True path of the requested video file.
/// * `relative_path` - The string representation of the relative video path.
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
/// * `config` - Configuration data for the application.
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `info` - Query string from the request.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the track file content or an error response.
#[get("/track")]
pub async fn track(config: web::Data<Arc<squire::settings::Config>>,
                   request: HttpRequest, info: web::Query<Payload>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    squire::logger::log_connection(&request);
    log::debug!("{}", auth_response.detail);
    log::debug!("Track requested: {}", &info.file);
    let filepath = Path::new(&config.video_source).join(&info.file);
    log::debug!("Track file lookup: {}", &filepath.to_string_lossy());
    match std::fs::read_to_string(&filepath) {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/plain")
            .body(content),
        Err(_) => HttpResponse::NotFound().json(routes::auth::DetailError {
            detail: format!("'{}' was not found", &info.file)
        })
    }
}

/// Create an `HttpResponse` based on the context built and rendered template.
///
/// # Arguments
///
/// * `landing` - `Template` retrieved from the configuration container.
/// * `serializable` - `HashMap` that can be serialized into a single String block which will be rendered.
fn render_content(landing: Template, serializable: HashMap<&str, &String>) -> HttpResponse {
    return match landing.render(serializable) {
        Ok(response_body) => {
            HttpResponse::build(StatusCode::OK)
                .content_type("text/html; charset=utf-8").body(response_body)
        }
        Err(err) => {
            log::error!("{}", err);
            HttpResponse::FailedDependency().json("Failed to render content.")
        }
    }
}

/// Handles requests for the '/stream/{video_path:.*}' endpoint, serving video files and directories.
///
/// # Arguments
///
/// * `config` - Configuration data for the application.
/// * `environment` - Configuration container for the loaded templates.
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `video_path` - The path parameter representing the video file or directory.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the video content or directory listing, or an error response.
#[get("/stream/{video_path:.*}")]
pub async fn stream(config: web::Data<Arc<squire::settings::Config>>,
                    environment: web::Data<Arc<Mutex<Environment<'static>>>>,
                    request: HttpRequest, video_path: web::Path<String>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    squire::logger::log_connection(&request);
    log::debug!("{}", auth_response.detail);
    let filepath = video_path.to_string();
    // True path of the video file
    let __target = config.video_source.join(&filepath);
    if !__target.exists() {
        return HttpResponse::NotFound().json(routes::auth::DetailError {
            detail: format!("'{}' was not found", filepath)
        });
    }
    // True path of the video file as a String
    let __target_str = __target.to_string_lossy().to_string();
    let __filename = __target.file_name().unwrap().to_string_lossy().to_string();
    let template = environment.lock().unwrap();
    if __target.is_file() {
        let landing = template.get_template("landing").unwrap();
        let rust_iter = squire::content::get_iter(&__target, &config.file_formats);
        let render_path = format!("/video?file={}", url_encode(&filepath));
        let prev = rust_iter.previous.unwrap_or_default();
        let next = rust_iter.next.unwrap_or_default();
        let mut context_builder = vec![
            ("video_title", &__filename),
            ("path", &render_path),
            ("previous", &prev),
            ("next", &next)
        ].into_iter().collect::<HashMap<_, _>>();
        if vec!["jpeg", "jpg", "png", "gif", "tiff", "tif", "bmp",
                "svg", "ico", "raw", "psd", "ai", "eps", "pdf"]
            .contains(&render_path.split('.').last()
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
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(listing.render(context!(
                files => listing_page.files, directories => listing_page.directories)
            ).unwrap());
    }
    log::error!("Something went really wrong");
    log::error!("Video Path: {}", filepath);
    log::error!("Target: {}", __target_str);
    HttpResponse::ExpectationFailed().json(routes::auth::DetailError {
        detail: format!("'{}' was neither a file nor a folder", filepath)
    })
}

/// Handles requests for the '/video' endpoint, serving video content for streaming.
///
/// # Arguments
///
/// * `config` - Configuration data for the application.
/// * `request` - A reference to the Actix web `HttpRequest` object.
/// * `info` - The query parameter containing the file information.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the video content or an error response.
#[get("/video")]
pub async fn streaming_endpoint(config: web::Data<Arc<squire::settings::Config>>,
                                request: HttpRequest, info: web::Query<Payload>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
    squire::logger::log_connection(&request);
    let host = request.connection_info().host().to_owned();
    let video_path = config.video_source.join(&info.file);
    if video_path.exists() {
        let file = actix_files::NamedFile::open_async(video_path).await.unwrap();
        // Check if the host is making a continued connection streaming the same file
        let mut tracker = constant::HOST_SERVE.lock().unwrap();
        if tracker.get(&host).unwrap() != &info.file {
            log::info!("Streaming {}", info.file);
            tracker.insert(request.connection_info().host().to_string(), info.file.to_string());
        }
        return file.into_response(&request);
    }
    let error = format!("File {:?} not found", video_path);
    log::error!("{}", error);
    HttpResponse::NotFound().body(error)
}
