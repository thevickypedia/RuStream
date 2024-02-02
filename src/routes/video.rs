use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, web};
use serde::Deserialize;

use crate::squire::settings;

lazy_static::lazy_static! {
    static ref HOST_SERVE: std::sync::Mutex<HashMap<String, String>> = std::sync::Mutex::new(HashMap::new());
}

#[derive(Deserialize)]
pub struct Payload {
    video_file: String,
}

#[get("/stream")]
pub async fn stream(config: web::Data<Arc<settings::Config>>,
                    req: HttpRequest, info: web::Query<Payload>) -> HttpResponse {
    let host = req.connection_info().host().to_owned();
    log::info!("Connection received from {}", host);  // todo: move to a function to log only once
    let video_path = config.video_source.join(&info.video_file);

    if video_path.exists() {
        let file = actix_files::NamedFile::open_async(video_path.clone()).await.unwrap();
        // Check if the host is making a continued connection streaming the same file
        let mut tracker = HOST_SERVE.lock().unwrap();
        if tracker.get(&host).is_some() && tracker.get(&host).unwrap() == &info.video_file {
            // logging is skipped since it is a continued streaming
        } else {
            log::info!("Streaming {}", info.video_file);
            tracker.insert(req.connection_info().host().to_string(), info.video_file.clone());
        }
        return file.into_response(&req);
    }
    let error = format!("File {:?} not found", video_path);
    log::error!("{}", error);
    HttpResponse::NotFound().body(error)
}
