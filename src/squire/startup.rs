use std::{env, path};
use std::sync::Arc;
use crate::constant;

use crate::squire::parser::Args;
use crate::squire::settings::Config;

pub fn get_binary() -> String {
    let binary = env::args().next().unwrap();
    path::Path::new(&binary).file_name().unwrap().to_str().unwrap().to_string()
}

pub fn init_logger(debug: bool, build_info: &constant::Cargo) {
    let logging_level;
    if debug {
        logging_level = format!("actix_web=debug,actix_server=info,{}=debug,{}=debug",
                                    build_info.binary, build_info.pkg_name);
        env::set_var("RUST_BACKTRACE", "1");
    } else {
        // set actix logging to warning mode since it becomes too noisy when streaming a giant video file
        logging_level = format!("actix_web=warn,actix_server=warn,{}=info,{}=info",
                                    build_info.binary, build_info.pkg_name);
        env::set_var("RUST_BACKTRACE", "0");
    }
    env::set_var("RUST_LOG", logging_level);
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
    let mut errors = "".to_owned();
    if !config.video_source.exists() || !config.video_source.is_dir() {
        let err1 = format!("\nvideo_source\n\tInput [{}] is not a valid directory [value=invalid]\n",
                           config.video_source.to_string_lossy());
        errors.push_str(&err1);
    }
    for (username, password) in &config.authorization {
        if username.len() < 4 {
            let err2 = format!(
                "\nauthorization\n\t[{}: {}] username should be at least 4 or more characters [value=invalid]\n",
                username, "*".repeat(password.len())
            );
            errors.push_str(&err2);
        }
        if password.len() < 8 {
            let err3 = format!(
                "\nauthorization\n\t[{}: {}] password should be at least 8 or more characters [value=invalid]\n",
                username, "*".repeat(password.len())
            );
            errors.push_str(&err3);
        }
    }
    if !errors.is_empty() {
        panic!("{}", errors);
    }
    config
}
