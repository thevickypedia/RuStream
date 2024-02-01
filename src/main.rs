#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{App, HttpServer, middleware, web};
use log;

mod routes;
mod squire;
mod render;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let args = squire::parser::arguments();
    squire::startup::init_logger(args.debug);
    let config = squire::startup::get_config(args);
    // Create a dedicated clone, since it will be used within closure
    let config_clone = config.clone();
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    let host = format!("{}:{}", config.video_host, config.video_port);
    log::info!("{} running on http://{} (Press CTRL+C to quit)", env!("CARGO_PKG_NAME"), host);
    HttpServer::new(move || {
        App::new()  // Creates a new Actix web application
            .app_data(web::Data::new(config_clone.clone()))
            .wrap(middleware::Logger::default())  // Adds a default logger middleware to the application
            .service(routes::basics::health)  // Registers a service for handling requests
            .service(routes::basics::status)
            .service(routes::basics::root)
            .service(routes::video::stream)
    })
        .workers(config.workers as usize)
        .max_connections(config.max_connections as usize)
        .bind(host)?
        .run()
        .await
}
