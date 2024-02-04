use actix_web::HttpRequest;

use crate::constant;

pub fn log_connection(request: &HttpRequest) {
    let mut tracker = constant::HOST_SERVE.lock().unwrap();
    let host = request.connection_info().host().to_owned();
    if tracker.get(&host).is_none() {
        tracker.insert(request.connection_info().host().to_string(), "".to_string());
        log::info!("Connection received from {}", host);
        if let Some(user_agent) = request.headers().get("user-agent") {
            log::info!("User agent: {}", user_agent.to_str().unwrap())
        }
    }
}
