use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::{Duration, OffsetDateTime};
use serde::Serialize;

use crate::routes::authenticator;
use crate::squire::settings;
use crate::constant;

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
        let mapping = mapped.unwrap();
        let payload = serde_json::to_string(&mapping).unwrap();
        let mut cookie = Cookie::build("session_token", constant::FERNET.encrypt(payload.as_bytes()))
            .http_only(true)
            .finish();
        let mut expiration = OffsetDateTime::now_utc();
        expiration += Duration::seconds(config.session_duration as i64);
        cookie.set_expires(expiration);
        log::info!("Session for '{}' will be valid until {}", mapping.get("username").unwrap(), expiration);
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
pub async fn home(_config: web::Data<Arc<settings::Config>>,
                  request: HttpRequest) -> HttpResponse {
    authenticator::verify_token(request);
    HttpResponse::Ok().finish()
}
