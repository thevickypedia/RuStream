use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use serde::Deserialize;

use crate::squire;
use crate::routes;

lazy_static::lazy_static! {
    static ref HOST_SERVE: std::sync::Mutex<HashMap<String, String>> = std::sync::Mutex::new(HashMap::new());
}

#[derive(Deserialize)]
pub struct Payload {
    video_file: String,
}

#[get("/stream/{video_path:.*}")]
pub async fn stream(config: web::Data<Arc<squire::settings::Config>>,
                    request: HttpRequest, video_path: web::Path<String>) -> HttpResponse {
    let auth_response = routes::authenticator::verify_token(request, &config);
    if auth_response.ok {
        log::debug!("{}", auth_response.detail);
        let target = config.video_source.join(video_path);
        if target.exists() {
            // todo: return landing page
        } else {
            return HttpResponse::NotFound().json(routes::auth::DetailError {
                detail: format!("'{}' was not found", video_path)
            })
        }
        return HttpResponse::Ok().finish();
    }
    let mut response = HttpResponse::Found();
    // Set to the lowest possible second since deletion is not an option
    let age = Duration::new(1, 0);
    let cookie = Cookie::build("detail", auth_response.detail)
        .http_only(true).max_age(age).finish();
    response.cookie(cookie);
    response.append_header(("Location", "/error"));
    return response.finish();
}

#[get("/video")]
pub async fn streaming_endpoint(config: web::Data<Arc<squire::settings::Config>>,
                                req: HttpRequest, info: web::Query<Payload>) -> HttpResponse {
    let host = req.connection_info().host().to_owned();
    log::info!("Connection received from {}", host);  // todo: move to a function to log only once
    let video_path = config.video_source.join(&info.video_file);

    if video_path.exists() {
        let file = actix_files::NamedFile::open_async(video_path).await.unwrap();
        // Check if the host is making a continued connection streaming the same file
        let mut tracker = HOST_SERVE.lock().unwrap();
        if tracker.get(&host).is_some() && tracker.get(&host).unwrap() == &info.video_file {
            // logging is skipped since it is a continued streaming
        } else {
            log::info!("Streaming {}", info.video_file);
            tracker.insert(req.connection_info().host().to_string(), info.video_file.to_string());
        }
        return file.into_response(&req);
    }
    let error = format!("File {:?} not found", video_path);
    log::error!("{}", error);
    HttpResponse::NotFound().body(error)
}
