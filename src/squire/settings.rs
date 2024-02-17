use std::{path, thread};
use std::collections::HashMap;
use std::net::ToSocketAddrs;

use serde::{Deserialize, Serialize};

/// Represents the configuration parameters for RuStream.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// Dictionary of key-value pairs for authorization (username and password).
    pub authorization: HashMap<String, String>,
    /// Source path for video files.
    pub video_source: path::PathBuf,

    /// Host IP address for video hosting.
    #[serde(default = "default_video_host")]
    pub video_host: String,
    /// Port number for hosting the application.
    #[serde(default = "default_video_port")]
    pub video_port: i32,
    /// Duration of a session in seconds.
    #[serde(default = "default_session_duration")]
    pub session_duration: i32,
    /// List of supported video file formats.
    #[serde(default = "default_file_formats")]
    pub file_formats: Vec<String>,

    /// Number of worker threads to spin up the server.
    #[serde(default = "default_workers")]
    pub workers: i32,
    /// Maximum number of concurrent connections.
    #[serde(default = "default_max_connections")]
    pub max_connections: i32,
    /// List of websites (supports regex) to add to CORS configuration.
    #[serde(default = "default_website")]
    pub website: Vec<String>,

    // Path to the full certificate chain file for SSL certificate
    #[serde(default="default_ssl")]
    pub cert_file: path::PathBuf,
    // Path to the private key file for SSL certificate
    #[serde(default="default_ssl")]
    pub key_file: path::PathBuf
}

/// Returns the default value for ssl files
fn default_ssl() -> path::PathBuf { path::PathBuf::new() }

/// Returns the default video host based on the local machine's IP address.
fn default_video_host() -> String {
    let hostname = "localhost";
    match (hostname, 0).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.find(|a| a.is_ipv4()) {
                return addr.ip().to_string();
            }
        }
        Err(err) => {
            log::error!("Error resolving hostname: {}", err);
        }
    }
    "localhost".to_string()
}

/// Returns the default video port (8000).
fn default_video_port() -> i32 {
    8000
}

/// Returns the default session duration (3600 seconds).
fn default_session_duration() -> i32 {
    3600
}

/// Returns the default supported file formats (.mp4 and .mov).
///
/// Set as public, since this function is re-used in `startup.rs`
pub fn default_file_formats() -> Vec<String> {
    // todo: remove the dot (.)
    vec![".mp4".to_string(), ".mov".to_string()]
}

/// Returns the default number of worker threads (half of logical cores).
fn default_workers() -> i32 {
    let logical_cores = thread::available_parallelism();
    match logical_cores {
        Ok(cores) => cores.get() as i32 / 2,
        Err(err) => {
            log::error!("{}", err);
            3
        }
    }
}

/// Returns the default maximum number of concurrent connections (300).
fn default_max_connections() -> i32 {
    300
}

/// Returns an empty list as the default website (CORS configuration).
fn default_website() -> Vec<String> {
    Vec::new()
}
