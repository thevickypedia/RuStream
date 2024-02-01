#[macro_use]
extern crate actix_web;

use std::{env, path};
use std::io;
use std::sync::Arc;

use actix_web::{App, HttpServer, middleware, web};
use serde_json::Result;
use log;

use crate::squire::Config;

mod routes;
mod squire;
mod parser;

pub async fn get_binary() -> String {
    let binary = env::args().next().unwrap();
    path::Path::new(&binary).file_name().unwrap().to_str().unwrap().to_string()
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let binary = get_binary().await;
    let args = parser::arguments();
    if args.debug {
        let logging_level = format!("actix_web=debug,actix_server=info,{}=debug", binary);
        env::set_var("RUST_LOG", logging_level);
        env::set_var("RUST_BACKTRACE", "1");
    } else {
        // set actix logging to warning mode since it becomes too noisy when streaming a giant video file
        let logging_level = format!("actix_web=warn,actix_server=warn,{}=info", binary);
        env::set_var("RUST_LOG", logging_level);
        env::set_var("RUST_BACKTRACE", "0");
    }
    env_logger::init();
    let filename;
    if args.filename.is_empty() {
        filename = "config.json".to_string()
    } else {
        filename = args.filename
    }
    let config;
    if std::path::Path::new(&filename).exists() {
        match std::fs::read_to_string(&filename) {
            Ok(content) => {
                let result: Result<Config> = serde_json::from_str(&content);
                match result {
                    Ok(raw_config) => {
                        config = Arc::new(raw_config);
                    }
                    Err(err) => {
                        println!("{:?}", content);
                        panic!("Error deserializing JSON: {}", err);
                    }
                }
            }
            Err(err) => {
                panic!("Error reading file: {}", err);
            }
        }
    } else {
        panic!("\nfilename\n\tInput [{}] is not a valid filepath [value=missing]\n", filename)
    }
    let arc_config = config.clone();
    /*
        || syntax is creating a closure that serves as the argument to the HttpServer::new() method.
        The closure is defining the configuration for the Actix web server.
        The purpose of the closure is to configure the server before it starts listening for incoming requests.
     */
    let host = format!("{}:{}", config.video_host, config.video_port);
    log::info!("{} running on http://{} (Press CTRL+C to quit)", env!("CARGO_PKG_NAME"), host);
    HttpServer::new(move || {
        App::new()  // Creates a new Actix web application
            .app_data(web::Data::new(arc_config.clone()))
            .wrap(middleware::Logger::default())  // Adds a default logger middleware to the application
            .service(routes::basics::health)  // Registers a service for handling requests
            .service(routes::basics::status)
            .service(routes::video::stream)
    })
        .workers(config.workers as usize)
        .max_connections(config.max_connections as usize)
        .bind(host)?
        .run()
        .await
}
