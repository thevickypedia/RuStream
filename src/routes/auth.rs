use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::http::StatusCode;
use itertools::Itertools;
use minijinja::context;
use serde::Serialize;

use crate::{constant, template, routes, squire};

#[derive(Serialize)]
struct RedirectResponse {
    redirect_url: String,
}

#[derive(Serialize)]
pub struct DetailError {
    pub detail: String,
}

#[post("/login")]
pub async fn login(config: web::Data<Arc<squire::settings::Config>>,
                   request: HttpRequest) -> HttpResponse {
    let mapped = routes::authenticator::verify_login(&request, &config);
    if mapped.is_none() {
        return HttpResponse::Unauthorized().json(DetailError {
            detail: "Incorrect username or password".to_string()
        });
    }
    squire::logger::log_connection(&request);
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
    response
}

#[get("/logout")]
pub async fn logout(config: web::Data<Arc<squire::settings::Config>>,
                    request: HttpRequest) -> HttpResponse {
    let host = request.connection_info().host().to_owned();
    let template = constant::ENV.lock().unwrap();
    let logout_template = template.get_template("logout").unwrap();
    let mut response = HttpResponse::build(StatusCode::OK);
    response.content_type("text/html; charset=utf-8");
    let rendered;
    let auth_response = routes::authenticator::verify_token(&request, &config);
    log::debug!("Session Validation Response: {}", auth_response.detail);
    if auth_response.username != "NA" {
        log::info!("{} from {} attempted to logged out", auth_response.username, host);
    }
    if auth_response.ok {
        let mut tracker = constant::HOST_SERVE.lock().unwrap();
        if tracker.get(&host).is_some() {
            tracker.remove(&host);
        } else {
            log::warn!("Session information for {} was not stored or no video was played", host);
        }
        rendered = logout_template.render(context!(detail => "You have been logged out successfully.")).unwrap();
        // Set to the lowest possible second since deletion is not an option
        let age = Duration::new(0, 1);
        let cookie = Cookie::build("session_token", "boo")
            .http_only(true).max_age(age).finish();
        response.cookie(cookie);
    } else {
        log::debug!("No stored session found for {}", host);
        rendered = logout_template.render(
            context!(detail => "You are not logged in. Please click the button below to proceed.",
                show_login => true)
        ).unwrap();
    }
    // response.finish() is not required since setting the body will close the response
    response.body(rendered)
}

#[get("/home")]
pub async fn home(config: web::Data<Arc<squire::settings::Config>>,
                  request: HttpRequest) -> HttpResponse {
    // todo: investigate why home page takes longer to load while sub folders render quickly
    // todo: cache this page to render faster
    let auth_response = routes::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        let mut response = HttpResponse::Found();
        // Set to the lowest possible second since deletion is not an option
        let age = Duration::new(0, 1);
        let cookie = Cookie::build("detail", auth_response.detail)
            .http_only(true).max_age(age).finish();
        response.cookie(cookie);
        response.append_header(("Location", "/error"));
        return response.finish();
    }
    squire::logger::log_connection(&request);
    log::debug!("{}", auth_response.detail);
    // https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.collect_tuple
    let file_format = config.file_formats.iter().collect_tuple().unwrap();
    let args = (config.video_source.to_string_lossy().to_string(), file_format);
    let listing_page = squire::fileio::get_all_stream_content(args);
    let template = constant::ENV.lock().unwrap();
    let listing = template.get_template("listing").unwrap();
    return HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(listing.render(context!(
                files => listing_page.files, directories => listing_page.directories)).unwrap()
        );
}

/// Error response endpoint where the users are redirected in case of issues with session-token
///
/// Uses the cookie set during redirect to pick the right HTML page and insert response within.
#[get("/error")]
pub async fn error(request: HttpRequest) -> HttpResponse {
    if let Some(detail) = request.cookie("detail") {
        let template = constant::ENV.lock().unwrap();
        let session = template.get_template("session").unwrap();
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(session.render(context!(reason => detail.value())).unwrap());
    }
    return HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(template::UNAUTHORIZED);
}
