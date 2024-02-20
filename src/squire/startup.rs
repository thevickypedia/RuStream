use std::env;
use std::sync::Arc;

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
/// # Returns
///
/// An `Arc` of the Config struct containing the application configuration.
pub fn get_config() -> Arc<Config> {
    let env_file = env::var("env_file").unwrap_or(".env".to_string());
    let env_file_path = env::current_dir()
        .unwrap_or_default()
        .join(env_file);
    match dotenv::from_path(env_file_path.as_path()) {
        Ok(_) => {
            // todo: fix envy parsing or parse manually
            // todo: neatly format the message in panic
            // println!("Env vars loaded successfully");
            // let authorization_str = env::var("authorization").expect("authorization not set in .env");
            // let authorization: HashMap<String, String> =
            //     serde_json::from_str(&authorization_str).expect("Error parsing JSON");
            // println!("AUTH::{:?}", authorization);

            match envy::from_env::<Config>() {
                Ok(config) => {
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
                    return Arc::new(config);
                }
                Err(err) => panic!("Error parsing environment variables: {}", err)
            }
        }
        Err(err) => panic!("Error loading environment variables: {}", err)
    }
}
