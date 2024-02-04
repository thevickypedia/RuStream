use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use actix_web::http::StatusCode;
use itertools::Itertools;
use minijinja::context;
use serde::Deserialize;
use url::form_urlencoded;

use crate::{squire, constant};
use crate::routes;

#[derive(Deserialize)]
pub struct Payload {
    file: String,
}

#[get("/stream/{video_path:.*}")]
pub async fn stream(config: web::Data<Arc<squire::settings::Config>>,
                    request: HttpRequest, video_path: web::Path<String>) -> HttpResponse {
    let auth_response = routes::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        let mut response = HttpResponse::Found();
        // Set to the lowest possible second since deletion is not an option
        let age = Duration::new(1, 0);
        let cookie = Cookie::build("detail", auth_response.detail)
            .http_only(true).max_age(age).finish();
        response.cookie(cookie);
        response.append_header(("Location", "/error"));
        return response.finish();
    }
    squire::logger::log_connection(&request);
    log::debug!("{}", auth_response.detail);
    let target = config.video_source.join(video_path.to_string());
    let target_str = target.to_string_lossy().to_string();
    if !target.exists() {
        return HttpResponse::NotFound().json(routes::auth::DetailError {
            detail: format!("'{}' was not found", video_path)
        });
    }
    let template = constant::ENV.lock().unwrap();
    if target.is_file() {
        let landing = template.get_template("landing").unwrap();
        let render_path = format!("/video?file={}", form_urlencoded::byte_serialize(target_str
            .as_bytes()).collect::<Vec<_>>().join(""));
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(landing.render(context!(video_title => video_path.to_string(), path => render_path)).unwrap());
    } else if target.is_dir() {
        let child_dir = target.iter().last().unwrap().to_string_lossy().to_string();
        let file_format = config.file_formats.iter().collect_tuple().unwrap();
        let args = (target_str, child_dir, file_format);
        let listing_page = squire::fileio::get_dir_stream_content(args);
        let listing = template.get_template("listing").unwrap();
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(listing.render(context!(
                files => listing_page.files, directories => listing_page.directories)
            ).unwrap());
    }
    log::error!("Something went really wrong");
    log::error!("Video Path: {}", video_path.to_string());
    log::error!("Target: {}", target_str);
    HttpResponse::ExpectationFailed().json(routes::auth::DetailError {
        detail: format!("'{}' was neither a file nor a folder", video_path)
    })
}

#[get("/video")]
pub async fn streaming_endpoint(config: web::Data<Arc<squire::settings::Config>>,
                                request: HttpRequest, info: web::Query<Payload>) -> HttpResponse {
    let auth_response = routes::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        let mut response = HttpResponse::Found();
        // Set to the lowest possible second since deletion is not an option
        let age = Duration::new(1, 0);
        let cookie = Cookie::build("detail", auth_response.detail)
            .http_only(true).max_age(age).finish();
        response.cookie(cookie);
        response.append_header(("Location", "/error"));
        return response.finish();
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
