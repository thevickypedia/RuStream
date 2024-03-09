use std::path::{Path, PathBuf};

use actix_multipart::form::{
    MultipartForm,
    tempfile::TempFile,
};
use actix_web::HttpResponse;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/upload")]
pub async fn save_files(MultipartForm(form): MultipartForm<UploadForm>) -> HttpResponse {
    for file in form.files {
        let filename = file.file_name.unwrap();
        let path = format!("uploads/{}", filename);
        log::info!("Saving to {path}");
        file.file.persist(path).unwrap();
    }
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <h3>Files have been uploaded successfully!!</h3>
        </body>
    </html>"#;
    HttpResponse::Ok().body(html)
}

#[get("/upload")]
pub async fn upload_files() -> HttpResponse {
    let upload_path = Path::new("uploads");
    if !upload_path.exists() {
        match std::fs::create_dir("uploads") {
            Ok(_) => {
                log::info!("Created uploads directory successfully");
            }
            Err(err) => {
                log::error!("{}", err);
            }
        }
    }
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;
    HttpResponse::Ok().body(html)
}
