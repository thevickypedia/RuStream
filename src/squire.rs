use std::{env, path};
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub authorization: HashMap<String, String>,
    pub video_source: path::PathBuf,

    pub video_host: String,
    pub video_port: i32,
    pub session_duration: i32,
    pub file_formats: Vec<String>,

    pub workers: i32,
    #[serde(default = "default_max_connections")]
    pub max_connections: i32,
    pub website: Vec<String>,
    pub auto_thumbnail: bool,
}

pub async fn get_binary() -> String {
    let binary = env::args().next().unwrap();
    path::Path::new(&binary).file_name().unwrap().to_str().unwrap().to_string()
}

pub fn default_max_connections() -> i32 {
    return 300
}
