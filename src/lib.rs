#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{App, HttpServer, middleware, web};
use rand::prelude::SliceRandom;

mod squire;
mod template;
mod constant;
mod routes;

pub async fn start() -> io::Result<()> {
    let args = squire::parser::arguments();

    let build_info = constant::build_info();
    squire::startup::init_logger(args.debug, &build_info);
    println!("Welcome to RuStream[{}] - {}", build_info.pkg_version, build_info.description);
    let arts = vec![squire::ascii_art::DOG, squire::ascii_art::DOLPHIN, squire::ascii_art::HORSE];
    println!("{}", arts.choose(&mut rand::thread_rng()).unwrap().to_string());

    let config = squire::startup::get_config(args);
    // Create a dedicated clone, since it will be used within closure
    let config_clone = config.clone();
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    let host = format!("{}:{}", config.video_host, config.video_port);
    log::info!("{} [workers:{}] running on http://{} (Press CTRL+C to quit)", env!("CARGO_PKG_NAME"), config.workers, host);
    HttpServer::new(move || {
        App::new()  // Creates a new Actix web application
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
