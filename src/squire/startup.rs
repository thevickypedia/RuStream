use std;
use crate::constant::Cargo;
use crate::squire;

use crate::squire::settings;

/// Initializes the logger based on the provided debug flag and cargo information.
///
/// # Arguments
///
/// * `debug` - A flag indicating whether to enable debug mode for detailed logging.
/// * `crate_name` - Name of the crate loaded during compile time.
pub fn init_logger(debug: bool, crate_name: &String) {
    if debug {
        std::env::set_var("RUST_LOG", format!(
            "actix_web=debug,actix_server=info,{}=debug", crate_name
        ));
        std::env::set_var("RUST_BACKTRACE", "1");
    } else {
        // Set Actix logging to warning mode since it becomes too noisy when streaming large files
        std::env::set_var("RUST_LOG", format!(
            "actix_web=warn,actix_server=warn,{}=info", crate_name
        ));
        std::env::set_var("RUST_BACKTRACE", "0");
    }
    env_logger::init();
}

/// Extracts the mandatory env vars by key and parses it as `HashMap<String, String>` and `PathBuf`
///
/// # Returns
///
/// Returns a tuple of `HashMap<String, String>` and `PathBuf`.
///
/// # Panics
///
/// If the value is missing or if there is an error parsing the `HashMap`
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
    let media_source_str = match std::env::var("media_source") {
        Ok(val) => val,
        Err(_) => {
            panic!(
                "\nmedia_source\n\texpected a directory path, received null [value=missing]\n",
            );
        }
    };
    (authorization, std::path::PathBuf::from(media_source_str))
}

/// Extracts the env var by key and parses it as a `bool`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<bool>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
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

/// Extracts the env var by key and parses it as a `i32`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<i32>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
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

/// Extracts the env var by key and parses it as a `Vec<String>`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<Vec<String>>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
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

/// Extracts the env var by key and parses it as a `PathBuf`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an option of `PathBuf` if the value is available.
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

/// Handler that's responsible to parse all the env vars.
///
/// # Returns
///
/// Instantiates the `Config` struct with the required parameters.
fn load_env_vars() -> settings::Config {
    let (authorization, media_source) = mandatory_vars();
    let debug = parse_bool("debug").unwrap_or(settings::default_debug());
    let media_host = std::env::var("media_host").unwrap_or(settings::default_media_host());
    let media_port = parse_i32("media_port").unwrap_or(settings::default_media_port());
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
        media_source,
        debug,
        media_host,
        media_port,
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

/// Validates all the required environment variables with the required settings.
///
/// # Returns
///
/// Returns the `Config` struct containing the required parameters.
fn validate_vars() -> settings::Config {
    let config = load_env_vars();
    let mut errors = "".to_owned();
    if !config.media_source.exists() || !config.media_source.is_dir() {
        let err1 = format!(
            "\nmedia_source\n\tInput [{}] is not a valid directory [value=invalid]\n",
            config.media_source.to_string_lossy()
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

/// Retrieves the environment variables and parses as the data-type specified in Config struct.
///
/// # Returns
///
/// Converts the config struct into an `Arc` and returns it.
pub fn get_config(cargo: &Cargo) -> std::sync::Arc<settings::Config> {
    let mut env_file = squire::parser::arguments(cargo);
    if env_file.is_empty() {
        env_file = std::env::var("env_file")
            .unwrap_or(std::env::var("ENV_FILE")
                .unwrap_or(".env".to_string()));
    }
    let env_file_path = std::env::current_dir()
        .unwrap_or_default()
        .join(env_file);
    let _ = dotenv::from_path(env_file_path.as_path());
    std::sync::Arc::new(validate_vars())
}
