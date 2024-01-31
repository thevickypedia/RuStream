use std::path::PathBuf;

use actix_web::{HttpRequest, HttpResponse};

#[get("/stream")]
pub async fn stream(req: HttpRequest) -> HttpResponse {
    log::info!("Connection received from {}", req.connection_info().host());
    // todo: toss around the loaded and validated config, between modules
    let video_path = PathBuf::from("/path/to/video.mp4");
    if video_path.exists() {
        let file = actix_files::NamedFile::open_async(video_path.clone()).await.unwrap();
        return file.into_response(&req)
    }
    let error = format!("File {:?} not found", video_path);
    log::error!("{}", error);
    return HttpResponse::NotFound().body(error)
}
