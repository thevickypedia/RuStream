use std::sync::{Arc, Mutex};

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::http::StatusCode;
use minijinja;
use serde::Serialize;

use crate::{constant, squire};

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
///
/// # Returns
///
/// * `200` - HttpResponse with a `session_token` and redirect URL to the `/home` entrypoint.
/// * `401` - HttpResponse with an error message for failed authentication.
#[post("/login")]
pub async fn login(config: web::Data<Arc<squire::settings::Config>>, request: HttpRequest) -> HttpResponse {
    let verified = squire::authenticator::verify_login(&request, &config);
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

    let cookie_duration = Duration::seconds(config.session_duration as i64);
    let expiration = OffsetDateTime::now_utc() + cookie_duration;
    let base_cookie = Cookie::build("session_token", encrypted_payload)
        .http_only(true)
        .same_site(SameSite::Strict)
        .max_age(cookie_duration)
        .expires(expiration);

    let cookie = if config.secure_session {
        log::info!("Marking 'session_token' cookie as secure!!");
        base_cookie.secure(true).finish()
    } else {
        base_cookie.finish()
    };
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
///
/// # Returns
///
/// Returns an `HTTPResponse` with the cookie for `session_token` reset if available.
#[get("/logout")]
pub async fn logout(config: web::Data<Arc<squire::settings::Config>>,
                    environment: web::Data<Arc<Mutex<minijinja::Environment<'static>>>>,
                    request: HttpRequest) -> HttpResponse {
    let host = request.connection_info().host().to_owned();
    let template = environment.lock().unwrap();
    let logout_template = template.get_template("logout").unwrap();
    let mut response = HttpResponse::build(StatusCode::OK);
    response.content_type("text/html; charset=utf-8");

    let rendered;
    let auth_response = squire::authenticator::verify_token(&request, &config);
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
        rendered = logout_template.render(minijinja::context!(detail => "You have been logged out successfully.")).unwrap();

        let mut cookie = Cookie::new("session_token", "");
        cookie.set_same_site(SameSite::Strict);
        cookie.make_removal();
        response.cookie(cookie);
    } else {
        log::debug!("No stored session found for {}", host);
        rendered = logout_template.render(
            minijinja::context!(detail => "You are not logged in. Please click the button below to proceed.",
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
///
/// # Returns
///
/// * `200` - Returns an `HTTPResponse` with the home/listing page if session token is valid.
/// * `401` - HttpResponse with an error message for failed authentication.
#[get("/home")]
pub async fn home(config: web::Data<Arc<squire::settings::Config>>,
                  environment: web::Data<Arc<Mutex<minijinja::Environment<'static>>>>,
                  request: HttpRequest) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config);
    if !auth_response.ok {
        return failed_auth(auth_response, &config);
    }
    squire::logger::log_connection(&request);
    log::debug!("{}", auth_response.detail);

    let listing_page = squire::content::get_all_stream_content(&config);
    let template = environment.lock().unwrap();
    let listing = template.get_template("listing").unwrap();

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(
            listing.render(minijinja::context!(
                files => listing_page.files, directories => listing_page.directories)
            ).unwrap()
        )
}

/// Handles the error endpoint, rendering the appropriate HTML page based on session issues.
///
/// # Arguments
///
/// * `request` - Actix HttpRequest containing information about the incoming request.
///
/// # Returns
///
/// HttpResponse with either a session expiry or unauthorized message.
#[get("/error")]
pub async fn error(environment: web::Data<Arc<Mutex<minijinja::Environment<'static>>>>,
                   request: HttpRequest) -> HttpResponse {
    let template = environment.lock().unwrap();
    if let Some(detail) = request.cookie("detail") {
        log::info!("Error response for /error: {}", detail.value());
        let session = template.get_template("session").unwrap();
        return HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(session.render(minijinja::context!(reason => detail.value())).unwrap());
    }

    log::info!("Sending unauthorized response for /error");
    let unauthorized = template.get_template("unauthorized").unwrap();
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(unauthorized.render(minijinja::context!()).unwrap())  // no arguments to render
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
pub fn failed_auth(auth_response: squire::authenticator::AuthToken,
                   config: &squire::settings::Config) -> HttpResponse {
    let mut response = HttpResponse::build(StatusCode::FOUND);
    let detail = auth_response.detail;
    let age = Duration::new(3, 0);
    let base_cookie = Cookie::build("detail", detail)
        .path("/error")
        .http_only(true)
        .same_site(SameSite::Strict)
        .max_age(age);
    let cookie = if config.secure_session {
        log::debug!("Marking 'detail' cookie as secure!!");
        base_cookie.secure(true).finish()
    } else {
        base_cookie.finish()
    };
    response.cookie(cookie);
    response.append_header(("Location", "/error"));
    response.finish()
}
