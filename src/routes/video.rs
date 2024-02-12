use std::path::PathBuf;
use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use actix_web::http::StatusCode;
use itertools::Itertools;
use minijinja::context;
use serde::Deserialize;
use url::form_urlencoded;

use crate::{constant, squire};
use crate::routes;
use crate::routes::authenticator::AuthToken;

/// Represents the payload structure for deserializing data from the request query parameters.
#[derive(Deserialize)]
pub struct Payload {
    file: String,
}

/// Represents the paths and filenames for subtitles, including both SRT and VTT formats.
struct Subtitles {
    srt: PathBuf,
    srt_file: String,
    vtt: PathBuf,
    vtt_file: String,
}

/// Constructs a `Subtitles` struct based on the provided `target` path and `target_str`.
///
/// # Arguments
///
/// * `target` - The target path for the subtitles.
/// * `target_str` - The string representation of the target path.
///
/// # Returns
///
/// Returns a `Subtitles` struct containing paths and filenames for both SRT and VTT subtitle files.
fn subtitles(target: PathBuf, target_str: String) -> Subtitles {
    let sfx = target_str.replace(&*target.extension().unwrap().to_string_lossy(), "");
    let mut srt = target.join(sfx);
    let mut vtt = srt.clone();
    srt.set_extension("srt");
    vtt.set_extension("vtt");
    let srt_file = srt.to_string_lossy().to_string();
    let vtt_file = vtt.to_string_lossy().to_string();
    Subtitles { srt, srt_file, vtt, vtt_file }
}

/// Constructs an `HttpResponse` for failed `session_token` verification.
///
/// # Arguments
///
/// * `auth_response` - The authentication response containing details of the failure.
///
/// # Returns
///
/// Returns an `HttpResponse` with a redirect, setting a cookie with the failure detail.
fn failed_auth(auth_response: AuthToken) -> HttpResponse {
    let mut response = HttpResponse::Found();
    // Set to the lowest possible second since deletion is not an option
    let age = Duration::new(1, 0);
    let cookie = Cookie::build("detail", auth_response.detail)
        .http_only(true).max_age(age).finish();
    response.cookie(cookie);
    response.append_header(("Location", "/error"));
    response.finish()
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
#[get("/track/{track_path:.*}")]
pub async fn track(config: web::Data<Arc<squire::settings::Config>>,
                   request: HttpRequest, track_path: web::Path<String>) -> HttpResponse {
    let auth_response = routes::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        return failed_auth(auth_response);
    }
    squire::logger::log_connection(&request);
    log::debug!("{}", auth_response.detail);
    let filepath = track_path.to_string();
    log::info!("File requested: {}", &filepath);
    match std::fs::read_to_string(&filepath) {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/plain")
            .body(content),
        Err(_) => HttpResponse::NotFound().json(routes::auth::DetailError {
            detail: format!("'{}' was not found", filepath)
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
        return failed_auth(auth_response);
    }
    squire::logger::log_connection(&request);
    log::debug!("{}", auth_response.detail);
    let filepath = video_path.to_string();
    let target = config.video_source.join(&filepath);
    let target_str = target.to_string_lossy().to_string();
    if !target.exists() {
        return HttpResponse::NotFound().json(routes::auth::DetailError {
            detail: format!("'{}' was not found", filepath)
        });
    }
    let template = constant::ENV.lock().unwrap();
    if target.is_file() {
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
        let args = (&target_str, file_format.unwrap());
        let iter = squire::fileio::get_iter(args);
        // https://rustjobs.dev/blog/how-to-url-encode-strings-in-rust/
        let render_path = format!("/video?file={}",
                                  form_urlencoded::byte_serialize(target_str.as_bytes())
                                      .collect::<Vec<_>>()
                                      .join(""));
        // Rust doesn't allow re-assignment, so might as well create a mutable variable
        let mut response_body = landing.render(context!(
                video_title => filepath, path => render_path, previous => iter.previous, next => iter.next
            )).unwrap();
        let subtitle = subtitles(target, target_str);
        if subtitle.vtt.exists() {
            let sfx_file = format!("/track/{}", subtitle.vtt_file);
            // let track_file = form_urlencoded::byte_serialize(sfx_file.as_bytes()).collect::<Vec<_>>().join("");
            response_body = landing.render(context!(
                video_title => filepath, path => render_path, previous => iter.previous, next => iter.next,
                track => sfx_file
            )).unwrap();
        } else if subtitle.srt.exists() {
            log::info!("Converting '{}' to '{}' for subtitles",
                subtitle.srt.file_name().unwrap().to_string_lossy(),
                subtitle.vtt.file_name().unwrap().to_string_lossy());
            if squire::fileio::srt_to_vtt(&subtitle.srt_file.to_string()) {
                log::debug!("Successfully converted srt to vtt file");
                let sfx_file = format!("/track/{}", subtitle.vtt_file);
                // let track_file = form_urlencoded::byte_serialize(vtt_file.as_bytes()).collect::<Vec<_>>().join("");
                response_body = landing.render(context!(
                    video_title => filepath, path => render_path, previous => iter.previous, next => iter.next,
                    track => sfx_file
                )).unwrap();
            }
        }
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8").body(response_body);
    } else if target.is_dir() {
        let child_dir = target.iter().last().unwrap().to_string_lossy().to_string();
        let default_values = squire::settings::default_file_formats();
        // https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.collect_tuple
        let _file_format = config.file_formats.iter().collect_tuple();
        let file_format = if _file_format.is_none() {
            log::debug!("CRITICAL::Failed to extract tuple from {:?}", config.file_formats);
            default_values.iter().collect_tuple()
        } else {
            _file_format
        };
        let args = (target_str, child_dir, file_format.unwrap());
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
    log::error!("Target: {}", target_str);
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
        return failed_auth(auth_response);
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
