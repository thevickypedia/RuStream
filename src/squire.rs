use std::collections::HashMap;
use std::path;

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
    #[serde(default = "default_auto_thumbnail")]
    pub auto_thumbnail: bool,
}


pub fn default_video_host() -> String { "localhost".to_string() }

pub fn default_video_port() -> i32 { 8000 }

pub fn default_session_duration() -> i32 { 3600 }

pub fn default_file_formats() -> Vec<String> {
    let mut formats = Vec::new();
    formats.push(".mp4".to_string());
    formats.push(".mov".to_string());
    formats
}

pub fn default_workers() -> i32 { 3 }

pub fn default_max_connections() -> i32 { 300 }

pub fn default_website() -> Vec<String> { Vec::new() }

pub fn default_auto_thumbnail() -> bool { true }
