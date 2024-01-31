use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use squire::Server;

use crate::squire;

#[derive(Deserialize)]
pub struct Payload {
    video_file: String,
}

#[get("/stream")]
pub async fn stream(config: web::Data<Arc<Server>>, req: HttpRequest, info: web::Query<Payload>) -> HttpResponse {
    let video_path = config.video_source.join(&info.video_file);
    log::info!("Connection received from {} requesting {:?}", req.connection_info().host(), info.video_file);
    if video_path.exists() {
        let file = actix_files::NamedFile::open_async(video_path.clone()).await.unwrap();
        log::info!("Streaming {}", info.video_file);
        return file.into_response(&req);
    }
    let error = format!("File {:?} not found", video_path);
    log::error!("{}", error);
    return HttpResponse::NotFound().body(error);
}
