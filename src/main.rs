#[macro_use]
extern crate actix_web;

use std::env;
use std::io;
use std::net::ToSocketAddrs;

use actix_web::{App, HttpServer, middleware};
// use serde_json::Value::String;
use log;

mod routes;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // let crate_name = env!("CARGO_PKG_NAME");  // fixme: enable when converting to package
    // let logging_level = format!("actix_web=debug,actix_server=info,{}=debug", crate_name);
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,stream=debug");
    env_logger::init();
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    HttpServer::new(|| {
        App::new()  // Creates a new Actix web application
            .wrap(middleware::Logger::default())  // Adds a default logger middleware to the application
            .service(routes::basics::health)  // Registers a service for handling requests
            .service(routes::basics::status)
    })
        .workers(12)  // todo: load from env var
        .max_connections(100)  // todo: load from env var
        .bind("0.0.0.0:4576")?  // todo: load from env var
        .run()
        .await
}
