use std::path::{Path, PathBuf};
use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::StatusCode;
use itertools::Itertools;
use minijinja::context;
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
    let mut srt = true_path.clone();
    let mut vtt = true_path.clone();
    srt.set_extension("srt");
    vtt.set_extension("vtt");

    // Set vtt extension to the relative path, so it could be used as a parameter in HTML
    let mut vtt_filepath = PathBuf::new().join(relative_path);
    vtt_filepath.set_extension("vtt");
    let vtt_file = vtt_filepath.to_string_lossy().to_string();

    Subtitles { srt, vtt, vtt_file }
}

/// Handles requests for the '/track/{track_path:.*}' endpoint, serving track files.
///
/// # Arguments
///
/// * `config` - The configuration settings.
/// * `request` - The HTTP request.
/// * `track_path` - The path parameter representing the track file.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the track file content or an error response.
#[get("/track")]
pub async fn track(config: web::Data<Arc<squire::settings::Config>>,
                   request: HttpRequest, info: web::Query<Payload>) -> HttpResponse {
    let auth_response = routes::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response);
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

/// Handles requests for the '/stream/{video_path:.*}' endpoint, serving video files and directories.
///
/// # Arguments
///
/// * `config` - The configuration settings.
/// * `request` - The HTTP request.
/// * `video_path` - The path parameter representing the video file or directory.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the video content or directory listing, or an error response.
#[get("/stream/{video_path:.*}")]
pub async fn stream(config: web::Data<Arc<squire::settings::Config>>,
                    request: HttpRequest, video_path: web::Path<String>) -> HttpResponse {
    let auth_response = routes::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response);
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
    let template = constant::ENV.lock().unwrap();
    if __target.is_file() {
        let landing = template.get_template("landing").unwrap();
        let default_values = squire::settings::default_file_formats();
        // https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.collect_tuple
        let _file_format = config.file_formats.iter().collect_tuple();
        let file_format = if _file_format.is_none() {
            log::debug!("CRITICAL::Failed to extract tuple from {:?}", config.file_formats);
            default_values.iter().collect_tuple()
        } else {
            _file_format
        };
        // full path required to read directory
        let args = (&__target_str, file_format.unwrap());
        let iter = squire::fileio::get_iter(args);
        // https://rustjobs.dev/blog/how-to-url-encode-strings-in-rust/
        let render_path = format!("/video?file={}", url_encode(&filepath));
        // Rust doesn't allow re-assignment, so might as well create a mutable variable
        // Load the default response body and re-construct with subtitles if present
        let mut response_body = landing.render(context!(
            video_title => &filepath, path => render_path,
            previous => &iter.previous,
            next => &iter.next,
            previous_title => &iter.previous,
            next_title => &iter.next,
        )).unwrap();
        let subtitle = subtitles(__target, &filepath);
        if subtitle.vtt.exists() {
            let sfx_file = format!("/track?file={}", url_encode(&subtitle.vtt_file));
            response_body = landing.render(context!(
                video_title => &filepath, path => render_path,
                previous => &iter.previous,
                next => &iter.next,
                previous_title => &iter.previous,
                next_title => &iter.next,
                track => sfx_file
            )).unwrap();
        } else if subtitle.srt.exists() {
            log::info!("Converting '{}' to '{}' for subtitles",
                subtitle.srt.file_name().unwrap().to_string_lossy(),
                subtitle.vtt.file_name().unwrap().to_string_lossy());
            if squire::fileio::srt_to_vtt(&subtitle.srt.to_string_lossy().to_string()) {
                log::debug!("Successfully converted srt to vtt file");
                let sfx_file = format!("/track?file={}", url_encode(&subtitle.vtt_file));
                response_body = landing.render(context!(
                    video_title => &filepath, path => render_path,
                    previous => &iter.previous,
                    next => &iter.next,
                    previous_title => &iter.previous,
                    next_title => &iter.next,
                    track => sfx_file
                )).unwrap();
            }
        }
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8").body(response_body);
    } else if __target.is_dir() {
        let child_dir = __target.iter().last().unwrap().to_string_lossy().to_string();
        let default_values = squire::settings::default_file_formats();
        // https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.collect_tuple
        let _file_format = config.file_formats.iter().collect_tuple();
        let file_format = if _file_format.is_none() {
            log::debug!("CRITICAL::Failed to extract tuple from {:?}", config.file_formats);
            default_values.iter().collect_tuple()
        } else {
            _file_format
        };
        let args = (__target_str, child_dir, file_format.unwrap());
        let listing_page = squire::fileio::get_dir_stream_content(args);
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
/// * `config` - The configuration settings.
/// * `request` - The HTTP request.
/// * `info` - The query parameter containing the file information.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the video content or an error response.
#[get("/video")]
pub async fn streaming_endpoint(config: web::Data<Arc<squire::settings::Config>>,
                                request: HttpRequest, info: web::Query<Payload>) -> HttpResponse {
    let auth_response = routes::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response);
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
