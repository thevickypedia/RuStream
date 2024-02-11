#[macro_use]
extern crate actix_web;

use std::io;

use actix_web::{App, HttpServer, middleware, web};
use rand::prelude::SliceRandom;

mod squire;
mod template;
mod constant;
mod routes;

pub async fn start() -> io::Result<()> {
    let cargo = constant::build_info();
    let args = squire::parser::arguments();

    squire::startup::init_logger(args.debug, &cargo);
    println!("{}[v{}] - {}", cargo.pkg_name, cargo.pkg_version, cargo.description);
    let arts = [squire::ascii_art::DOG, squire::ascii_art::DOLPHIN, squire::ascii_art::HORSE];
    println!("{}", arts.choose(&mut rand::thread_rng()).unwrap());

    let config = squire::startup::get_config(args);
    // Create a dedicated clone, since it will be used within closure
    let config_clone = config.clone();
    let host = format!("{}:{}", config.video_host, config.video_port);
    log::info!("{} [workers:{}] running on http://{} (Press CTRL+C to quit)",
        cargo.pkg_name, config.workers, host);
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    HttpServer::new(move || {
        App::new()  // Creates a new Actix web application
            .wrap(squire::middleware::get_cors(&config_clone.clone().website))
            .app_data(web::Data::new(config_clone.clone()))
            .wrap(middleware::Logger::default())  // Adds a default logger middleware to the application
            .service(routes::basics::health)  // Registers a service for handling requests
            .service(routes::basics::root)
            .service(routes::auth::login)
            .service(routes::auth::logout)
            .service(routes::auth::home)
            .service(routes::auth::error)
            .service(routes::video::track)
            .service(routes::video::stream)
            .service(routes::video::streaming_endpoint)
    })
        .workers(config.workers as usize)
        .max_connections(config.max_connections as usize)
        .bind(host)?
        .run()
        .await
}
