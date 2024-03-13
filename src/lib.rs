#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate actix_web;

use std::io;

use actix_web::{App, HttpServer, middleware, web};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

/// Module for the structs and functions called during startup.
mod constant;
/// Module for all the API entry points.
mod routes;
/// Module to store all the helper functions.
mod squire;
/// Module to load all the templates for the UI.
mod templates;

/// Contains entrypoint and initializer settings to trigger the asynchronous `HTTPServer`
///
/// # Examples
///
/// ```no_run
/// #[actix_rt::main]
/// async fn main() {
///     match rustream::start().await {
///         Ok(_) => {
///             println!("RuStream session terminated")
///         }
///         Err(err) => {
///             eprintln!("Error starting rustream: {}", err)
///         }
///     }
/// }
/// ```
pub async fn start() -> io::Result<()> {
    let cargo = constant::build_info();
    let config = squire::startup::get_config(&cargo);

    squire::startup::init_logger(config.debug, config.utc_logging, &cargo.crate_name);
    println!("{}[v{}] - {}", &cargo.pkg_name, &cargo.pkg_version, &cargo.description);
    squire::ascii_art::random();

    if config.secure_session {
        log::warn!(
            "Secure session is turned on! This means that the server can ONLY be hosted via HTTPS or localhost"
        );
    }
    // Create a dedicated clone, since it will be used within closure
    let config_clone = config.clone();
    let host = format!("{}:{}", config.media_host, config.media_port);
    log::info!("{} [workers:{}] running on http://{} (Press CTRL+C to quit)",
        &cargo.pkg_name, &config.workers, &host);
    let jinja = templates::environment();
    let fernet = constant::fernet_object();
    let session = constant::session_info();
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    let max_payload_size = 10 * 1024 * 1024 * 1024; // 10 GB
    let application = move || {
        App::new()  // Creates a new Actix web application
            .app_data(web::Data::new(config_clone.clone()))
            .app_data(web::Data::new(jinja.clone()))
            .app_data(web::Data::new(fernet.clone()))
            .app_data(web::Data::new(session.clone()))
            .app_data(web::PayloadConfig::default().limit(max_payload_size))
            .wrap(squire::middleware::get_cors(config_clone.websites.clone()))
            .wrap(middleware::Logger::default())  // Adds a default logger middleware to the application
            .service(routes::basics::health)  // Registers a service for handling requests
            .service(routes::basics::root)
            .service(routes::auth::login)
            .service(routes::auth::logout)
            .service(routes::auth::home)
            .service(routes::auth::error)
            .service(routes::media::track)
            .service(routes::media::stream)
            .service(routes::media::streaming_endpoint)
            .service(routes::upload::upload_files)
            .service(routes::upload::save_files)
    };
    let server = HttpServer::new(application)
        .workers(config.workers as usize)
        .max_connections(config.max_connections as usize);
    // Reference: https://actix.rs/docs/http2/
    if config.cert_file.exists() && config.key_file.exists() {
        log::info!("Binding SSL certificate to serve over HTTPS");
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file(&config.key_file, SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file(&config.cert_file).unwrap();
        server.bind_openssl(host, builder)?
            .run()
            .await
    } else {
        server.bind(host)?
            .run()
            .await
    }
}
