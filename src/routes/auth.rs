use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::cookie::Cookie;
use actix_web::http::header::HeaderValue;

use crate::squire;
use crate::squire::settings;
use serde::Serialize;

struct Credentials {
    username: String,
    signature: String,
    timestamp: String,
}

#[derive(Serialize)]
struct RedirectResponse {
    redirect_url: String
}

#[derive(Serialize)]
struct DetailError {
    detail: String
}

/// Extracts credentials from the authorization header in the following steps
///
/// 1. Decodes the base64 encoded header
///
/// 2. Splits it into 3 parts with first one being the username followed by the signature and timestamp
///
/// 3. Converts the username from hex into a string.
fn extract_credentials(authorization: Option<&HeaderValue>) -> Credentials {
    let header = authorization.unwrap().to_str().unwrap().to_string();
    // base64 encoded in JavaScript using inbuilt btoa function
    let decoded_auth = squire::secure::base64_decode(&header).to_string();
    let vector: Vec<&str> = decoded_auth.split(",").collect();
    // Decode hex username into string to retrieve password from config file
    let username = squire::secure::hex_decode(vector.get(0).unwrap());
    let signature = vector.get(1).unwrap().to_string();
    let timestamp = vector.get(2).unwrap().to_string();
    Credentials { username, signature, timestamp }
}

#[post("/login")]
pub async fn login(config: web::Data<Arc<settings::Config>>,
                   request: HttpRequest) -> HttpResponse {
    let authorization = request.headers().get("authorization");
    if authorization.is_some() {
        let credentials = extract_credentials(authorization);
        let password = config.authorization.get(&credentials.username);
        if password.is_some() {  // Check if the username is present in HashMap as key
            let message = format!("{}{}{}",
                                  squire::secure::hex_encode(&credentials.username),
                                  squire::secure::hex_encode(password.unwrap()),
                                  credentials.timestamp);
            // Create a new signature with hex encoded username and password stored in config file as plain text
            let expected_signature = squire::secure::calculate_hash(message);
            if expected_signature == credentials.signature {
                log::info!("Signature matched for {}", credentials.username);
                let cookie = Cookie::build("session_token", "thiswillbechanged")
                    .http_only(true)
                    .finish();
                let mut response = HttpResponse::Ok().json(RedirectResponse {
                    redirect_url: "/home".to_string(),
                });
                response.add_cookie(&cookie).unwrap();
                return response;
            }
            log::warn!("{} entered bad credentials", credentials.username);
            return HttpResponse::Unauthorized().json(DetailError {
                detail: "Incorrect username or password".to_string()
            });
        }
        log::warn!("{} is not allowed", credentials.username);
        return HttpResponse::Unauthorized().json(DetailError {
            detail: "Incorrect username or password".to_string()
        });
    }
    return HttpResponse::Unauthorized().json(DetailError {
        detail: "Username and password are mandatory for authentication".to_string()
    });
}
