use std;
use std::ffi::OsStr;
use std::io::Write;

use chrono::{DateTime, Local, Utc};
use walkdir::WalkDir;

use crate::{constant, squire};
use crate::constant::Cargo;
use crate::squire::settings;

/// Initializes the logger based on the provided debug flag and cargo information.
///
/// # Arguments
///
/// * `debug` - A flag indicating whether to enable debug mode for detailed logging.
/// * `crate_name` - Name of the crate loaded during compile time.
pub fn init_logger(debug: bool, utc: bool, crate_name: &String) {
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
    if utc {
        env_logger::init();
    } else {
        env_logger::Builder::from_default_env()
            .format(|buf, record| {
                let local_time: DateTime<Local> = Local::now();
                writeln!(
                    buf,
                    "[{} {} {}] - {}",
                    local_time.format("%Y-%m-%dT%H:%M:%SZ"),
                    record.level(),
                    record.target(),
                    record.args()
                )
            })
            .init();
    }
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
    let utc_logging = parse_bool("utc_logging").unwrap_or(settings::default_utc_logging());
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
        utc_logging,
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

/// Get the current time in a specific format.
///
/// # Arguments
///
/// * `utc` - Boolean flag to return the time in UTC timezone.
///
/// # Returns
///
/// Returns the current datetime as a `String`.
fn get_time(utc: bool) -> String {
    if utc {
        Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    } else {
        Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
}

/// Validates the directory structure to ensure that the secure index is present in media source's root.
///
/// # Arguments
///
/// * `config` - Configuration data for the application.
/// * `cargo` - Package specific information loaded in a struct.
fn validate_dir_structure(config: &settings::Config, cargo: &Cargo) {
    let source = &config.media_source.to_string_lossy().to_string();
    let mut errors = String::new();
    for entry in WalkDir::new(&config.media_source).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        if entry_path.is_dir() && entry_path.to_str().unwrap().ends_with(constant::SECURE_INDEX) {
            let secure_index = entry_path.strip_prefix(source).unwrap();
            let depth = secure_index.iter().count();
            if depth != 1usize {
                let index_vec = secure_index.iter().collect::<Vec<_>>();
                let secure_dir = index_vec.last().unwrap();
                // secure_parent_path is the secure index's location
                let secure_parent_path = &index_vec[0..index_vec.len() - 1]
                    .join(OsStr::new(std::path::MAIN_SEPARATOR_STR));
                errors.push_str(&format!(
                    "\n{:?}\n\tSecure index directory [{:?}] should be at the root [{:?}] [depth={}, valid=1]\n\
                    \t> Hint: Either move {:?} within {:?}, [OR] set the 'media_source' to {:?}\n",
                    secure_index,
                    secure_dir,
                    config.media_source,
                    depth,
                    secure_dir,
                    config.media_source,
                    config.media_source.join(secure_parent_path)
                ));
            }
        }
    }
    if errors.is_empty() {
        for username in config.authorization.keys() {
            let secure_path = &config.media_source.join(format!("{}_{}", &username, constant::SECURE_INDEX));
            if !secure_path.exists() {
                match std::fs::create_dir(secure_path) {
                    Ok(_) => {
                        // keep formatting similar to logging
                        if config.utc_logging {
                            println!("[{}\x1b[32m INFO\x1b[0m  {}] '{}' has been created",
                                     get_time(config.utc_logging), cargo.crate_name,
                                     &secure_path.to_str().unwrap())
                        } else {
                            println!("[{} INFO  {}] '{}' has been created",
                                     get_time(config.utc_logging), cargo.crate_name,
                                     &secure_path.to_str().unwrap())
                        }
                    },
                    Err(err) => panic!("{}", err)
                }
            }
        }
    } else {
        panic!("{}", errors)
    }
}

/// Validates all the required environment variables with the required settings.
///
/// # Arguments
///
/// * `cargo` - Package specific information loaded in a struct.
///
/// # Returns
///
/// Returns the `Config` struct containing the required parameters.
fn validate_vars(cargo: &Cargo) -> settings::Config {
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
    validate_dir_structure(&config, cargo);
    config
}

/// Retrieves the environment variables and parses as the data-type specified in Config struct.
///
/// # Arguments
///
/// * `cargo` - Package specific information loaded in a struct.
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
    std::sync::Arc::new(validate_vars(cargo))
}
