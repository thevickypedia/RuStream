#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../README.md")]

#[macro_use]
extern crate actix_web;

use std::io;
use std::sync::Arc;

use actix_web::{App, HttpServer, middleware, web};
use fernet::Fernet;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

/// Module to load all the static values and required structs during startup.
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

    squire::startup::init_logger(config.debug, &cargo.crate_name);
    println!("{}[v{}] - {}", &cargo.pkg_name, &cargo.pkg_version, &cargo.description);
    squire::ascii_art::random();

    if config.secure_session {
        log::warn!(
            "Secure session is turned on! This means that the server can ONLY be hosted via HTTPS or localhost"
        );
    }
    let jinja_env = templates::environment();
    // Create a dedicated clone, since it will be used within closure
    let config_clone = config.clone();
    let jinja_clone = jinja_env.clone();
    let host = format!("{}:{}", config.media_host, config.media_port);
    log::info!("{} [workers:{}] running on http://{} (Press CTRL+C to quit)",
        &cargo.pkg_name, &config.workers, &host);
    let fernet = Arc::new(Fernet::new(&squire::fernet_key()).unwrap());
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    let application = move || {
        App::new()  // Creates a new Actix web application
            .app_data(web::Data::new(config_clone.clone()))
            .app_data(web::Data::new(jinja_clone.clone()))
            .app_data(web::Data::new(fernet.clone()))
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
