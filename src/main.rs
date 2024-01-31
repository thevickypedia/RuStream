#[macro_use]
extern crate actix_web;

use std::env;
use std::io;

use actix_web::{App, HttpServer, middleware, web};
// use serde_json::Value::String;
use log;

mod routes;
mod squire;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    let binary = squire::get_binary().await;
    let logging_level = format!("actix_web=debug,actix_server=info,{}=debug", binary);
    env::set_var("RUST_LOG", logging_level);
    // todo: take debug as an cmdline arg and enable this if flag is passed
    env::set_var("RUST_BACKTRACE", "0");
    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,stream=debug");
    env_logger::init();
    let arc_config = squire::Server::config();
    let config = arc_config.clone();
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    let host = format!("0.0.0.0:{}", config.port);
    log::info!("{} running on http://{} (Press CTRL+C to quit)", env!("CARGO_PKG_NAME"), host);
    HttpServer::new(move || {
        App::new()  // Creates a new Actix web application
            .app_data(web::Data::new(arc_config.clone()))
            .wrap(middleware::Logger::default())  // Adds a default logger middleware to the application
            .service(routes::basics::health)  // Registers a service for handling requests
            .service(routes::basics::status)
            .service(routes::video::stream)
    })
        .workers(config.workers)
        .max_connections(config.max_connections)
        .bind(host)?
        .run()
        .await
}
