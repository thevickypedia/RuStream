use std::{env, path};
use std::sync::Arc;

use crate::squire::parser::Args;
use crate::squire::settings::Config;

/// Initializes the logger based on the provided debug flag and cargo information.
///
/// # Arguments
///
/// * `debug` - A flag indicating whether to enable debug mode for detailed logging.
/// * `cargo` - A reference to the Cargo struct containing information about the application.
pub fn init_logger(debug: bool, crate_name: &String) {
    if debug {
        env::set_var("RUST_LOG", format!(
            "actix_web=debug,actix_server=info,{}=debug", crate_name
        ));
        env::set_var("RUST_BACKTRACE", "1");
    } else {
        // Set Actix logging to warning mode since it becomes too noisy when streaming a giant video file.
        env::set_var("RUST_LOG", format!(
            "actix_web=warn,actix_server=warn,{}=info", crate_name
        ));
        env::set_var("RUST_BACKTRACE", "0");
    }
    env_logger::init();
}

/// Retrieves the configuration from the provided command-line arguments.
///
/// # Arguments
///
/// * `args` - Command-line arguments provided to the application.
///
/// # Returns
///
/// An `Arc` of the Config struct containing the application configuration.
pub fn get_config(args: Args) -> Arc<Config> {
    let filename = if args.filename.is_empty() {
        log::warn!("Missing 'filename' argument, assuming default ('config.json')");
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
                        log::error!("Error deserializing JSON");
                        panic!("\n{}\n", err);
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
        let err1 = format!(
            "\nvideo_source\n\tInput [{}] is not a valid directory [value=invalid]\n",
            config.video_source.to_string_lossy()
        );
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
