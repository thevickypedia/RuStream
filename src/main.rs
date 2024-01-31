#[macro_use]
extern crate actix_web;

use std::env;
use std::io;

use actix_web::{App, HttpServer, middleware};
// use serde_json::Value::String;
use log;

mod routes;
mod squire;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    let binary = squire::get_binary().await;
    let logging_level = format!("actix_web=debug,actix_server=info,{}=debug", binary);
    env::set_var("RUST_LOG", logging_level);
    // env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,stream=debug");
    env_logger::init();
    let workers = squire::getenv("workers",
                                 squire::getenv("WORKERS", 3).await).await as usize;
    let max_connections = squire::getenv("max_connections",
                                         squire::getenv("MAX_CONNECTIONS", 100).await).await as usize;
    let port = squire::getenv("port",
                              squire::getenv("PORT", 8000).await).await as usize;
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    let host = format!("0.0.0.0:{}", port);
    log::info!("{} running on http://{} (Press CTRL+C to quit)", env!("CARGO_PKG_NAME"), host);
    HttpServer::new(|| {
        App::new()  // Creates a new Actix web application
            .wrap(middleware::Logger::default())  // Adds a default logger middleware to the application
            .service(routes::basics::health)  // Registers a service for handling requests
            .service(routes::basics::status)
    })
        .workers(workers)
        .max_connections(max_connections)
        .bind(host)?
        .run()
        .await
}
