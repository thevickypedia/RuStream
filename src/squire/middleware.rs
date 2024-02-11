use actix_cors::Cors;
use actix_web::http::header;

pub fn get_cors(website: &Vec<String>) -> Cors {
    let mut origins = vec!["http://localhost.com".to_string(),
                           "https://localhost.com".to_string()];
    if !website.is_empty() {
        origins.append(&mut website.to_owned());
    }
    // Create a clone to append /* to each endpoint, and further extend the same vector
    let cloned = origins.clone().into_iter().map(|x| format!("{}/{}", x, "*"));
    origins.extend(cloned);
    let mut cors = Cors::default()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600);  // maximum time (in seconds) for which this CORS request maybe cached
    for origin in origins {
        cors = cors.allowed_origin(&origin)
    }
    cors
}
