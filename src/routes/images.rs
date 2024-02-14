use std::path::PathBuf;
use std::string::ToString;

use actix_web::{HttpRequest, HttpResponse, web};
use lazy_static::lazy_static;

use crate::{routes, squire};

lazy_static! {
    static ref IMAGES: PathBuf = PathBuf::new().join(env!("CARGO_MANIFEST_DIR")).join("src").join("images");
}

/// An Actix web handler for serving images based on the requested filename.
///
/// # Parameters
///
/// * `request` - Actix HttpRequest containing information about the incoming request.
/// * `filename` - Extracted from the request path, the name of the requested image file.
///
/// # Returns
///
/// - `HttpResponse`: Responds with the requested image content if found, or raises a 404.
#[get("/images/{filename:.*}")]
pub async fn image_endpoint(request: HttpRequest, filename: web::Path<String>) -> HttpResponse {
    // Log the incoming connection for monitoring purposes
    squire::logger::log_connection(&request);
    log::debug!("Image requested: {}", &filename);

    // Define allowed image file types
    let allowed_types = ["jpeg", "jpg", "png", "gif"];

    // Extract the file extension from the requested filename
    let extension = filename.split('.').last().unwrap_or("NA");

    // Determine the image file type and format
    let filetype = if allowed_types.contains(&extension) {
        format!("image/{}", &extension)
    } else {
        // Return a BadRequest response if the file type is not allowed
        return HttpResponse::BadRequest().json(routes::auth::DetailError {
            detail: format!("'{}' is not an allowed filetype", &extension)
        });
    };

    // Construct the full file path for the requested image
    let filepath = IMAGES.join(filename.to_string());
    log::debug!("Image file lookup: {}", &filepath.to_string_lossy());

    // Attempt to read the image content from the file
    match web::block(|| std::fs::read(filepath)).await {
        // Respond with the image content if successful
        Ok(image_content) => HttpResponse::Ok()
            .content_type(filetype)
            .body(image_content.unwrap()),
        // Return a NotFound response if the file is not found
        Err(_) => HttpResponse::NotFound().json(routes::auth::DetailError {
            detail: format!("'{}' was not found", &filename)
        })
    }
}
