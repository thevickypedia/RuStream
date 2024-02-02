use std::sync::Arc;
use actix_web::cookie::Cookie;
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::http::header::HeaderValue;
use actix_web::HttpRequest;
use actix_web::web::Data;

use crate::squire;
use crate::squire::settings;

struct Credentials {
    username: String,
    signature: String,
    timestamp: String,
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

pub fn verify_login(request: HttpRequest, config: Data<Arc<settings::Config>>) -> Option<Cookie<'static>> {
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
                let mut cookie = Cookie::build("session_token", "thiswillbechanged")
                    .http_only(true)
                    .finish();
                let mut expiration = OffsetDateTime::now_utc();
                expiration += Duration::seconds(config.session_duration as i64);
                cookie.set_expires(expiration);
                log::info!("Session for '{}' will be valid until {}", credentials.username, expiration);
                return Some(cookie);
            } else {
                log::warn!("{} entered bad credentials", credentials.username);
            }
        } else {
            log::warn!("{} is not allowed", credentials.username);
        }
    }
    return None;
}
