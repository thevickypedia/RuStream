use std::path::Path;
use std::sync::Arc;

use actix_multipart::form::{
    MultipartForm,
    tempfile::TempFile,
};
use actix_web::{HttpRequest, HttpResponse, web, http};
use fernet::Fernet;

use crate::{constant, routes, squire};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/upload")]
pub async fn save_files(request: HttpRequest,
                        fernet: web::Data<Arc<Fernet>>,
                        session: web::Data<Arc<constant::Session>>,
                        config: web::Data<Arc<squire::settings::Config>>,
                        MultipartForm(form): MultipartForm<UploadForm>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
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
pub async fn upload_files(request: HttpRequest,
                          fernet: web::Data<Arc<Fernet>>,
                          config: web::Data<Arc<squire::settings::Config>>,
                          template: web::Data<Arc<minijinja::Environment<'static>>>,
                          session: web::Data<Arc<constant::Session>>) -> HttpResponse {
    let auth_response = squire::authenticator::verify_token(&request, &config, &fernet, &session);
    if !auth_response.ok {
        return routes::auth::failed_auth(auth_response, &config);
    }
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
    let landing = template.get_template("upload").unwrap();
    HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(landing.render(minijinja::context!()).unwrap())  // no arguments to render
}
