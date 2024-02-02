use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use serde::Serialize;

use crate::routes::authenticator;
use crate::squire::settings;

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
    let cookie = authenticator::verify_login(request, config);
    if cookie.is_some() {
        let mut response = HttpResponse::Ok().json(RedirectResponse {
            redirect_url: "/home".to_string(),
        });
        response.add_cookie(&cookie.unwrap()).unwrap();
        return response;
    }
    return HttpResponse::Unauthorized().json(DetailError {
        detail: "Incorrect username or password".to_string()
    });
}

// #[post("/home")]
// pub async fn home(config: web::Data<Arc<settings::Config>>,
//                   request: HttpRequest) -> HttpResponse {}
