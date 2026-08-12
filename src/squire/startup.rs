use std;
use std::io::Write;

use chrono::{DateTime, Local, Utc};

use crate::{constant, squire};
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
    let authorization = match std::env::var("authorization") {
        Ok(val) => match serde_json::from_str::<std::collections::HashMap<String, String>>(&val) {
            Ok(parsed) => parsed,
            Err(_) => {
                panic!(
                    "\nauthorization\n\terror parsing JSON [value=invalid]\n",
                );
            }
        },
        Err(_) => settings::default_authorization(),
    };
    let media_source = match std::env::var("media_source") {
        Ok(val) => std::path::PathBuf::from(val),
        Err(_) => settings::default_media_source(),
    };
    (authorization, media_source)
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

/// Extracts the env var by key and parses it as a `i64`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<i64>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
fn parse_i64(key: &str) -> Option<i64> {
    match std::env::var(key) {
        Ok(val) => match val.parse() {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected i64, received '{}' [value=invalid]\n", key, val);
            }
        },
        Err(_) => None,
    }
}

/// Extracts the env var by key and parses it as a `u16`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<u16>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
fn parse_u16(key: &str) -> Option<u16> {
    match std::env::var(key) {
        Ok(val) => match val.parse() {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected u16, received '{}' [value=invalid]\n", key, val);
            }
        },
        Err(_) => None,
    }
}

/// Extracts the env var by key and parses it as a `usize`
///
/// # Arguments
///
/// * `key` - Key for the environment variable.
///
/// # Returns
///
/// Returns an `Option<usize>` if the value is available.
///
/// # Panics
///
/// If the value is present, but it is an invalid data-type.
fn parse_usize(key: &str) -> Option<usize> {
    match std::env::var(key) {
        Ok(val) => match val.parse() {
            Ok(parsed) => Some(parsed),
            Err(_) => {
                panic!("\n{}\n\texpected usize, received '{}' [value=invalid]\n", key, val);
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

/// Normalizes the base URL to start with a single leading slash and no trailing slashes.
fn normalize_base_url(value: String) -> String {
    let mut base = value.trim().to_string();
    if !base.starts_with('/') {
        base.insert(0, '/');
    }
    base = base.trim_end_matches('/').to_string();
    if base.is_empty() {
        base = "/".to_string();
    }
    base
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

/// Parses the maximum payload size from human-readable memory format to bytes.
///
/// - `key` - Key for the environment variable.
///
/// ## See Also
///
/// - This function handles internal panic gracefully, in the most detailed way possible.
/// - Panic outputs are suppressed with a custom hook.
/// - Custom hook is set before wrapping the potentially panicking function inside `catch_unwind`.
/// - Custom hook is reset later, so the future panics and go uncaught.
/// - Error message from panic payload is also further processed, to get a detailed reason for panic.
///
/// # Returns
///
/// Returns an option of usize if the value is parsable and within the allowed size limit.
fn parse_max_payload(key: &str) -> Option<usize> {
    match std::env::var(key) {
        Ok(value) => {

            let custom_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_panic_info| {}));
            let result = std::panic::catch_unwind(|| parse_memory(&value));
            std::panic::set_hook(custom_hook);

            match result {
                Ok(output) => {
                    if let Some(value) = output {
                        Some(value)
                    } else {
                        panic!("\n{}\n\texpected format: '100 MB', received '{}' [value=invalid]\n",
                               key, value);
                    }
                }
                Err(panic_payload) => {
                    if let Some(&error) = panic_payload.downcast_ref::<&str>() {
                        panic!("\n{}\n\t{} [value=invalid]\n", key, error);
                    } else if let Some(error) = panic_payload.downcast_ref::<String>() {
                        panic!("\n{}\n\t{} [value=invalid]\n", key, error);
                    } else if let Some(error) = panic_payload.downcast_ref::<Box<dyn std::fmt::Debug + Send + 'static>>() {
                        panic!("\n{}\n\t{:?} [value=invalid]\n", key, error);
                    } else {
                        panic!("\n{}\n\tinvalid memory format! unable to parse panic payload [value=invalid]\n", key);
                    }
                }
            }
        }
        Err(_) => {
            None
        }
    }
}

fn parse_memory(memory: &str) -> Option<usize> {
    let value = memory.trim();
    let (size_str, unit) = value.split_at(value.len() - 2);
    let size: usize = match size_str.strip_suffix(' ').unwrap_or_default().parse() {
        Ok(num) => num,
        Err(_) => return None,
    };

    match unit.to_lowercase().as_str() {
        "zb" => Some(size * 1024 * 1024 * 1024 * 1024 * 1024),
        "tb" => Some(size * 1024 * 1024 * 1024 * 1024),
        "gb" => Some(size * 1024 * 1024 * 1024),
        "mb" => Some(size * 1024 * 1024),
        "kb" => Some(size * 1024),
        _ => None,
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
    let media_port = parse_u16("media_port").unwrap_or(settings::default_media_port());
    let session_duration = parse_i64("session_duration").unwrap_or(settings::default_session_duration());
    let file_formats = parse_vec("file_formats").unwrap_or(settings::default_file_formats());
    let workers = parse_usize("workers").unwrap_or(settings::default_workers());
    let max_connections = parse_usize("max_connections").unwrap_or(settings::default_max_connections());
    let websites = parse_vec("websites").unwrap_or(settings::default_websites());
    let secure_session = parse_bool("secure_session").unwrap_or(settings::default_secure_session());
    let ffmpeg_enabled = parse_bool("ffmpeg_enabled").unwrap_or(settings::default_ffmpeg_enabled());
    let key_file = parse_path("key_file").unwrap_or(settings::default_ssl());
    let cert_file = parse_path("cert_file").unwrap_or(settings::default_ssl());
    let max_payload_size = parse_max_payload("max_payload_size").unwrap_or(settings::default_max_payload_size());
    let base_url = match std::env::var("base_url") {
        Ok(val) => normalize_base_url(val),
        Err(_) => settings::default_base_url(),
    };
    settings::Config {
        authorization,
        media_source,
        debug,
        utc_logging,
        media_host,
        media_port,
        base_url,
        session_duration,
        file_formats,
        workers,
        max_connections,
        max_payload_size,
        websites,
        secure_session,
        key_file,
        cert_file,
        ffmpeg_enabled,
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

/// Validates all the required environment variables with the required settings.
///
/// # Arguments
///
/// * `metadata` - Struct containing metadata of the application.
///
/// # Returns
///
/// Returns the `Config` struct containing the required parameters.
fn warn_default(key: &str, detail: &str, utc: bool, crate_name: &String) {
    if utc {
        println!("[{}\x1b[33m WARN\x1b[0m  {}] '{}' was not provided, {}", get_time(utc), crate_name, key, detail)
    } else {
        println!("[{} WARN  {}] '{}' was not provided, {}", get_time(utc), crate_name, key, detail)
    }
}

fn validate_vars(metadata: &constant::MetaData) -> settings::Config {
    let config = load_env_vars();
    let authorization_provided = std::env::var("authorization").is_ok();
    let media_source_provided = std::env::var("media_source").is_ok();
    if !authorization_provided {
        warn_default("authorization", "using default credentials", config.utc_logging, &metadata.crate_name);
    }
    if !media_source_provided {
        warn_default("media_source", "using the default directory", config.utc_logging, &metadata.crate_name);
    }
    let mut errors = "".to_owned();
    if !config.media_source.exists() || !config.media_source.is_dir() {
        if !media_source_provided {
            match std::fs::create_dir_all(&config.media_source) {
                Ok(_) => {
                    if config.utc_logging {
                        println!("[{}\x1b[32m INFO\x1b[0m  {}] '{}' has been created",
                                 get_time(config.utc_logging), metadata.crate_name,
                                 config.media_source.to_string_lossy())
                    } else {
                        println!("[{} INFO  {}] '{}' has been created",
                                 get_time(config.utc_logging), metadata.crate_name,
                                 config.media_source.to_string_lossy())
                    }
                }
                Err(err) => errors.push_str(&format!(
                    "\nmedia_source\n\tUnable to create the default directory [{}]: {} [value=invalid]\n",
                    config.media_source.to_string_lossy(), err
                )),
            }
        } else {
            errors.push_str(&format!(
                "\nmedia_source\n\tInput [{}] is not a valid directory [value=invalid]\n",
                config.media_source.to_string_lossy()
            ));
        }
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
/// # Arguments
///
/// * `metadata` - Struct containing metadata of the application.
///
/// # Returns
///
/// Converts the config struct into an `Arc` and returns it.
pub fn get_config(metadata: &constant::MetaData) -> std::sync::Arc<settings::Config> {
    let mut env_file = squire::parser::arguments(metadata);
    if env_file.is_empty() {
        env_file = std::env::var("env_file")
            .unwrap_or(std::env::var("ENV_FILE")
                .unwrap_or(".env".to_string()));
    }
    let env_file_path = std::env::current_dir()
        .unwrap_or_default()
        .join(env_file);
    let _ = dotenv::from_path(env_file_path.as_path());
    std::sync::Arc::new(validate_vars(metadata))
}
