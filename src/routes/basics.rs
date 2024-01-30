use actix_web::HttpResponse;

#[get("/health")]
pub async fn health() -> HttpResponse {
    return HttpResponse::Ok().content_type("application/json").json("Healthy");
}

#[get("/status")]
pub async fn status() -> HttpResponse {
    return HttpResponse::Ok().content_type("application/json").json("Status");
}
