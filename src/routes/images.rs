use std::path::PathBuf;
use std::string::ToString;
use actix_web::{HttpRequest, HttpResponse, web};
use lazy_static::lazy_static;

use crate::{routes, squire};

lazy_static! {
    static ref IMAGES: PathBuf = PathBuf::new().join(env!("CARGO_MANIFEST_DIR")).join("src").join("images");
}

#[get("/images/{filename:.*}")]
pub async fn image_endpoint(request: HttpRequest, filename: web::Path<String>) -> HttpResponse {
    squire::logger::log_connection(&request);
    log::debug!("Image requested: {}", &filename);
    let allowed_types = ["jpeg", "jpg", "png", "gif"];
    let extension = filename.split('.').last().unwrap_or("NA");
    let filetype = if allowed_types.contains(&extension) {
        format!("image/{}", &extension)
    } else {
        return HttpResponse::BadRequest().json(routes::auth::DetailError {
            detail: format!("'{}' is not an allowed filetype", &extension)
        });
    };
    let filepath = IMAGES.join(filename.to_string());
    log::debug!("Image file lookup: {}", &filepath.to_string_lossy());
    match web::block(|| std::fs::read(filepath)).await {
        Ok(image_content) => HttpResponse::Ok()
            .content_type(filetype)
            .body(image_content.unwrap()),
        Err(_) => HttpResponse::NotFound().json(routes::auth::DetailError {
            detail: format!("'{}' was not found", &filename)
        })
    }
}
