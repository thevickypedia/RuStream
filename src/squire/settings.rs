use std::{path, thread};
use std::collections::HashMap;
use std::net::ToSocketAddrs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub authorization: HashMap<String, String>,
    pub video_source: path::PathBuf,

    #[serde(default = "default_video_host")]
    pub video_host: String,
    #[serde(default = "default_video_port")]
    pub video_port: i32,
    #[serde(default = "default_session_duration")]
    pub session_duration: i32,
    #[serde(default = "default_file_formats")]
    pub file_formats: Vec<String>,

    #[serde(default = "default_workers")]
    pub workers: i32,
    #[serde(default = "default_max_connections")]
    pub max_connections: i32,
    #[serde(default = "default_website")]
    pub website: Vec<String>,
}


pub fn default_video_host() -> String {
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

pub fn default_video_port() -> i32 { 8000 }

pub fn default_session_duration() -> i32 { 3600 }

pub fn default_file_formats() -> Vec<String> { vec![".mp4".to_string(), ".mov".to_string()] }

/// Set default workers to the number of physical cores (half of logical cores)
pub fn default_workers() -> i32 {
    let logical_cores = thread::available_parallelism();
    match logical_cores {
        Ok(cores) => cores.get() as i32 / 2,
        Err(err) => {
            log::error!("{}", err);
            3
        }
    }
}

pub fn default_max_connections() -> i32 { 300 }

pub fn default_website() -> Vec<String> { Vec::new() }
