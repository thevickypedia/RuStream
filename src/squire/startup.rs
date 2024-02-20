use std;

use crate::squire::settings;

/// Initializes the logger based on the provided debug flag and cargo information.
///
/// # Arguments
///
/// * `debug` - A flag indicating whether to enable debug mode for detailed logging.
/// * `cargo` - A reference to the Cargo struct containing information about the application.
pub fn init_logger(debug: bool, crate_name: &String) {
    if debug {
        std::env::set_var("RUST_LOG", format!(
            "actix_web=debug,actix_server=info,{}=debug", crate_name
        ));
        std::env::set_var("RUST_BACKTRACE", "1");
    } else {
        // Set Actix logging to warning mode since it becomes too noisy when streaming a giant video file.
        std::env::set_var("RUST_LOG", format!(
            "actix_web=warn,actix_server=warn,{}=info", crate_name
        ));
        std::env::set_var("RUST_BACKTRACE", "0");
    }
    env_logger::init();
}

fn mandatory_vars() -> (std::collections::HashMap<String, String>, std::path::PathBuf) {
    let authorization_str = match std::env::var("authorization") {
        Ok(val) => val,
        Err(_) => {
            panic!(
                "\nauthorization\n\texpected a HashMap, received null [value=missing]\n",
            );
        }
    };
    let authorization: std::collections::HashMap<String, String> =
        match serde_json::from_str(&authorization_str) {
            Ok(val) => val,
            Err(_) => {
                panic!(
                    "\nauthorization\n\terror parsing JSON [value=invalid]\n",
                );
            }
        };
    let video_source_str = match std::env::var("video_source") {
        Ok(val) => val,
        Err(_) => {
            panic!(
                "\nvideo_source\n\texpected a directory path, received null [value=missing]\n",
            );
        }
    };
    (authorization, std::path::PathBuf::from(video_source_str))
}

fn parse_bool(key: &str) -> Option<bool> {
    match std::env::var(key) {
        Ok(val) => match val.parse() {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected bool, received '{}' [value=invalid]\n", key, val);
            }
        },
        Err(_) => None,
    }
}

fn parse_i32(key: &str) -> Option<i32> {
    match std::env::var(key) {
        Ok(val) => match val.parse() {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected i32, received '{}' [value=invalid]\n", key, val);
            }
        },
        Err(_) => None,
    }
}

fn parse_vec(key: &str) -> Option<Vec<String>> {
    match std::env::var(key) {
        Ok(val) => match serde_json::from_str::<Vec<String>>(&val) {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected vec, received '{}' [value=invalid]\n", key, val);
            }
        },
        Err(_) => None,
    }
}

fn parse_path(key: &str) -> Option<std::path::PathBuf> {
    match std::env::var(key) {
        Ok(value) => {
            Some(std::path::PathBuf::from(value))
        }
        Err(_) => {
            None
        }
    }
}

fn load_env_vars() -> settings::Config {
    let (authorization, video_source) = mandatory_vars();
    let debug = parse_bool("debug").unwrap_or(settings::default_debug());
    let video_host = std::env::var("video_host").unwrap_or(settings::default_video_host());
    let video_port = parse_i32("video_port").unwrap_or(settings::default_video_port());
    let session_duration = parse_i32("session_duration").unwrap_or(settings::default_session_duration());
    let file_formats = parse_vec("file_formats").unwrap_or(settings::default_file_formats());
    let workers = parse_i32("workers").unwrap_or(settings::default_workers());
    let max_connections = parse_i32("max_connections").unwrap_or(settings::default_max_connections());
    let websites = parse_vec("websites").unwrap_or(settings::default_websites());
    let secure_session = parse_bool("secure_session").unwrap_or(settings::default_secure_session());
    let key_file = parse_path("key_file").unwrap_or(settings::default_ssl());
    let cert_file = parse_path("cert_file").unwrap_or(settings::default_ssl());
    settings::Config {
        authorization,
        video_source,
        debug,
        video_host,
        video_port,
        session_duration,
        file_formats,
        workers,
        max_connections,
        websites,
        secure_session,
        key_file,
        cert_file,
    }
}

/// Retrieves the configuration from the provided command-line arguments.
///
/// # Returns
///
/// An `Arc` of the Config struct containing the application configuration.
pub fn get_config() -> std::sync::Arc<settings::Config> {
    // todo: Update docs to be explicit about what `env_file` means
    let env_file = std::env::var("env_file").unwrap_or(".env".to_string());
    let env_file_path = std::env::current_dir()
        .unwrap_or_default()
        .join(env_file);
    match dotenv::from_path(env_file_path.as_path()) {
        Ok(_) => {
            let config = load_env_vars();
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
            return std::sync::Arc::new(config);
        }
        Err(err) => panic!("Error loading environment variables: {}", err)
    }
}
