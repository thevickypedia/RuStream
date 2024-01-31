use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};

use squire::Server;

use crate::squire;

#[get("/stream")]
pub async fn stream(config: web::Data<Arc<Server>>, req: HttpRequest) -> HttpResponse {
    // fixme: make me better
    let video_file = req.query_string().split('=').collect::<Vec<_>>().get(1).unwrap().to_string();
    let video_path = config.video_source.join(video_file.clone());
    log::info!("Connection received from {} accessing {:?}", req.connection_info().host(), video_file);
    if video_path.exists() {
        let file = actix_files::NamedFile::open_async(video_path.clone()).await.unwrap();
        return file.into_response(&req);
    }
    let error = format!("File {:?} not found", video_path);
    log::error!("{}", error);
    return HttpResponse::NotFound().body(error);
}
