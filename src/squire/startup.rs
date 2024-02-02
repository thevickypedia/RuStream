use std::{env, path};
use std::sync::Arc;

use crate::squire::parser::Args;
use crate::squire::settings::Config;

pub fn get_binary() -> String {
    let binary = env::args().next().unwrap();
    path::Path::new(&binary).file_name().unwrap().to_str().unwrap().to_string()
}

pub fn init_logger(debug: bool) {
    let binary = get_binary();
    if debug {
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
}

pub fn get_config(args: Args) -> Arc<Config> {
    let filename = if args.filename.is_empty() {
        "config.json".to_string()
    } else {
        args.filename
    };
    let config;
    if path::Path::new(&filename).exists() {
        match std::fs::read_to_string(&filename) {
            Ok(content) => {
                let result: serde_json::Result<Config> = serde_json::from_str(&content);
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
    config
}
