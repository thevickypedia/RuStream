use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::http::StatusCode;
use minijinja::{context, Environment};
use serde::Serialize;

use crate::{constant, render, squire};
use crate::routes::authenticator;
use crate::squire::settings;

#[derive(Serialize)]
struct RedirectResponse {
    redirect_url: String,
}

#[derive(Serialize)]
struct DetailError {
    detail: String,
}

#[post("/login")]
pub async fn login(config: web::Data<Arc<settings::Config>>,
                   request: HttpRequest) -> HttpResponse {
    let mapped = authenticator::verify_login(request, config.clone());
    if mapped.is_some() {
        let payload = serde_json::to_string(&mapped).unwrap();
        let mut cookie = Cookie::build("session_token", constant::FERNET.encrypt(payload.as_bytes()))
            .http_only(true)
            .finish();
        let mut expiration = OffsetDateTime::now_utc();
        expiration += Duration::seconds(config.session_duration as i64);
        cookie.set_expires(expiration);
        log::info!("Session for '{}' will be valid until {}", mapped.unwrap().get("username").unwrap(), expiration);
        let mut response = HttpResponse::Ok().json(RedirectResponse {
            redirect_url: "/home".to_string(),
        });
        response.add_cookie(&cookie).unwrap();
        return response;
    }
    HttpResponse::Unauthorized().json(DetailError {
        detail: "Incorrect username or password".to_string()
    })
}

#[get("/home")]
pub async fn home(config: web::Data<Arc<settings::Config>>,
                  request: HttpRequest) -> HttpResponse {
    let auth_response = authenticator::verify_token(request, config.clone());  // todo: check with &
    if auth_response.ok {
        log::debug!("{}", auth_response.detail);
        // todo: avoid hard coding index
        let file_format = (config.file_formats[0].to_string(), config.file_formats[1].to_string());
        let args = (config.video_source.to_string_lossy().to_string(), file_format);
        let listing_page = squire::fileio::get_py_content("get_all_stream_content", args);
        let mut env = Environment::new();
        env.add_template("listing", render::LISTING).unwrap();
        let template = env.get_template("listing").unwrap();
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(template.render(context!(
                files => listing_page.files, directories => listing_page.directories)).unwrap()
            );
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

/// Error response endpoint where the users are redirected in case of issues with session-token
///
/// Uses the cookie set during redirect to pick the right HTML page and insert response within.
#[get("/error")]
pub async fn error(request: HttpRequest) -> HttpResponse {
    if let Some(detail) = request.cookie("detail") {
        let mut env = Environment::new();
        env.add_template("session", render::SESSION).unwrap();
        let template = env.get_template("session").unwrap();
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(template.render(context!(reason => detail.value())).unwrap());
    }
    return HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(render::UNAUTHORIZED);
}
