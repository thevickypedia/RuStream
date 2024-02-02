use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use actix_web::http::header::HeaderValue;

use crate::squire;
use crate::squire::settings;

struct Credentials {
    username: String,
    signature: String,
    timestamp: String,
}

fn extract_credentials(authorization: Option<&HeaderValue>) -> Credentials {
    let header = authorization.unwrap().to_str().unwrap().to_string();
    let decoded_auth = squire::secure::base64_decode(header).to_string();
    let vector: Vec<&str> = decoded_auth.split(",").collect();
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
            let expected_signature = squire::secure::calculate_hash(message);
            if expected_signature == credentials.signature {
                log::info!("Signature matched for {}", credentials.username);
                return HttpResponse::Ok()  // todo: JSON response with cookie
                    .content_type("application/json")
                    .json("Authenticated Successfully");
            }
            log::warn!("{} entered bad credentials", credentials.username);
            return HttpResponse::Unauthorized().finish();
        }
        log::warn!("{} is not allowed", credentials.username);
        return HttpResponse::Unauthorized().finish();
    }
    return HttpResponse::Unauthorized().finish();
}
