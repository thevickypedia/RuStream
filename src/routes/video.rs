use actix_web::{HttpRequest, HttpResponse, web};

#[get("/stream")]
pub async fn stream(req: HttpRequest) -> HttpResponse {
    log::info!("Connection received from {}", req.connection_info().host());
    let data = "This is a streaming response.";
    let streaming_body = web::Bytes::from(data);
    let streaming = futures_util::stream::once(
        async move { Ok::<_, actix_web::Error>(streaming_body) }
    );
    HttpResponse::Ok().streaming(streaming)
}

