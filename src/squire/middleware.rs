use actix_cors::Cors;
use actix_web::http::header;

/// Configures and returns a CORS middleware based on provided website origins.
///
/// # Arguments
///
/// * `websites` - A vector of allowed website origins for CORS.
///
/// # Returns
///
/// A configured `Cors` middleware instance.
pub fn get_cors(websites: Vec<String>) -> Cors {
    let mut origins = vec!["http://localhost.com".to_string(), "https://localhost.com".to_string()];
    if !websites.is_empty() {
        origins.extend_from_slice(&websites);
    }
    // Create a clone to append /* to each endpoint, and further extend the same vector
    let cloned = origins.clone().into_iter().map(|x| format!("{}/{}", x, "*"));
    origins.extend(cloned);
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .allowed_header("secure-flag")
        .max_age(3600);  // Maximum time (in seconds) for which this CORS request may be cached
    for origin in origins {
        cors = cors.allowed_origin(&origin);
    }
    cors
}
