use std::sync::{Arc, Mutex};

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::http::StatusCode;
use itertools::Itertools;
use minijinja::{context, Environment};
use serde::Serialize;

use crate::{constant, routes, squire, jinja};
use crate::routes::authenticator::AuthToken;

/// Struct for representing a JSON Response with a redirect URL.
#[derive(Serialize)]
struct RedirectResponse {
    redirect_url: String,
}

/// Struct for representing detailed errors in JSON format.
#[derive(Serialize)]
pub struct DetailError {
    pub detail: String,
}

/// Handles the login endpoint, verifying credentials and creating session tokens.
///
/// # Arguments
///
/// * `config` - Configuration data for the application.
/// * `request` - Actix HttpRequest containing information about the incoming request.
#[post("/login")]
pub async fn login(config: web::Data<Arc<squire::settings::Config>>, request: HttpRequest) -> HttpResponse {
    let verified = routes::authenticator::verify_login(&request, &config);
    if let Err(err) = verified {
        let err_message = err.to_string();
        log::warn!("Error response::{}", err_message);
        return HttpResponse::Unauthorized().json(DetailError {
            detail: err_message
        });
    }

    let mapped = verified.unwrap();
    squire::logger::log_connection(&request);

    let payload = serde_json::to_string(&mapped).unwrap();
    let encrypted_payload = constant::FERNET.encrypt(payload.as_bytes());

    let mut cookie = Cookie::build("session_token", encrypted_payload)
        .http_only(true)
        .finish();

    let expiration = OffsetDateTime::now_utc() + Duration::seconds(config.session_duration as i64);
    cookie.set_expires(expiration);

    log::info!("Session for '{}' will be valid until {}", mapped.get("username").unwrap(), expiration);

    let mut response = HttpResponse::Ok().json(RedirectResponse {
        redirect_url: "/home".to_string(),
    });
    response.add_cookie(&cookie).unwrap();
    response
}

/// Handles the logout endpoint, logging out the user and rendering the appropriate HTML page.
///
/// # Arguments
///
/// * `config` - Configuration data for the application.
/// * `request` - Actix HttpRequest containing information about the incoming request.
#[get("/logout")]
pub async fn logout(config: web::Data<Arc<squire::settings::Config>>,
                    environment: web::Data<Arc<Mutex<Environment<'static>>>>,
                    request: HttpRequest) -> HttpResponse {
    let host = request.connection_info().host().to_owned();
    let template = environment.lock().unwrap();
    let logout_template = template.get_template("logout").unwrap();
    let mut response = HttpResponse::build(StatusCode::OK);
    response.content_type("text/html; charset=utf-8");

    let rendered;
    let auth_response = routes::authenticator::verify_token(&request, &config);
    log::debug!("Session Validation Response: {}", auth_response.detail);

    if auth_response.username != "NA" {
        log::info!("{} from {} attempted to log out", auth_response.username, host)
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
        let cookie = Cookie::build("session_token", "logout")
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

/// Handles the home endpoint, rendering the listing page for authenticated users.
///
/// # Arguments
///
/// * `config` - Configuration data for the application.
/// * `request` - Actix HttpRequest containing information about the incoming request.
#[get("/home")]
pub async fn home(config: web::Data<Arc<squire::settings::Config>>,
                  environment: web::Data<Arc<Mutex<Environment<'static>>>>,
                  request: HttpRequest) -> HttpResponse {
    let auth_response = routes::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        return failed_auth(auth_response);
    }
    squire::logger::log_connection(&request);
    log::debug!("{}", auth_response.detail);

    let default_values = squire::settings::default_file_formats();
    // https://docs.rs/itertools/latest/itertools/trait.Itertools.html#method.collect_tuple
    let _file_format = config.file_formats.iter().collect_tuple();
    let file_format = if _file_format.is_none() {
        log::debug!("CRITICAL::Failed to extract tuple from {:?}", config.file_formats);
        default_values.iter().collect_tuple()
    } else {
        _file_format
    };
    let args = (config.video_source.to_string_lossy().to_string(), file_format.unwrap());
    let listing_page = squire::fileio::get_all_stream_content(args);
    let template = environment.lock().unwrap();
    let listing = template.get_template("listing").unwrap();

    return HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(listing.render(context!(
                files => listing_page.files, directories => listing_page.directories)).unwrap()
        );
}

/// Handles the error endpoint, rendering the appropriate HTML page based on session issues.
///
/// # Arguments
///
/// * `request` - Actix HttpRequest containing information about the incoming request.
#[get("/error")]
pub async fn error(environment: web::Data<Arc<Mutex<Environment<'static>>>>,
                   request: HttpRequest) -> HttpResponse {
    if let Some(detail) = request.cookie("detail") {
        log::info!("Error response for /error: {}", detail.value());
        let template = environment.lock().unwrap();
        let session = template.get_template("session").unwrap();
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(session.render(context!(reason => detail.value())).unwrap());
    }

    log::info!("Sending unauthorized response for /error");
    return HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(jinja::get_content("unauthorized"));
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
pub fn failed_auth(auth_response: AuthToken) -> HttpResponse {
    let mut response = HttpResponse::build(StatusCode::FOUND);
    let detail = auth_response.detail;
    let age = Duration::new(3, 0);
    let cookie = Cookie::build("detail", detail)
        .path("/error")
        .http_only(true)
        .max_age(age)
        .finish();
    response.cookie(cookie);
    response.append_header(("Location", "/error"));
    response.finish()
}
