use actix_web::HttpRequest;

use crate::constant;

/// Logs connection information for an incoming HTTP request.
///
/// # Arguments
///
/// * `request` - A reference to the Actix web `HttpRequest` object.
///
/// This function logs the host and user agent information of the incoming connection.
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
