use std::{env, path};
use std::sync::Arc;

pub struct Server {
    pub video_source: path::PathBuf,
    pub workers: usize,
    pub max_connections: usize,
    pub port: usize,
}

fn getenv_int(key: &str, default: i32) -> i32 {
    if let Ok(value_lower) = env::var(key.to_lowercase()) {
        if let Ok(parsed_value) = value_lower.parse::<i32>() {
            return parsed_value;
        }
        panic!("\n{}\n\tInput is not valid i32 [value={}]\n", key, value_lower)
    }
    if let Ok(value_upper) = env::var(key.to_uppercase()) {
        if let Ok(parsed_value) = value_upper.parse::<i32>() {
            return parsed_value;
        }
        panic!("\n{}\n\tInput is not valid i32 [value={}]\n", key, value_upper)
    }
    default
}

fn getenv_str(key: &str, default: Option<String>) -> String {
    if let Ok(value_lower) = env::var(key.to_lowercase()) {
        return value_lower;
    }
    if let Ok(value_upper) = env::var(key.to_uppercase()) {
        return value_upper;
    }
    if let Some(default_value) = default {
        return default_value;
    }
    panic!("\n{}\n\tField required [value=missing]\n", key)
}


impl Server {
    pub fn config() -> Arc<Server> {
        let binding = getenv_str("video_source", None);
        let src_path = path::Path::new(&binding);
        if !src_path.exists() {
            panic!("\nvideo_source\n\tInput is not a valid path [value={:?}]\n", src_path)
        }
        Arc::new(Server {
            video_source: src_path.to_owned(),
            workers: getenv_int("workers", 3) as usize,
            max_connections: getenv_int("max_connections", 100) as usize,
            port: getenv_int("port", 8000) as usize,
        })
    }
}

pub async fn get_binary() -> String {
    let binary = env::args().next().unwrap();
    path::Path::new(&binary).file_name().unwrap().to_str().unwrap().to_string()
}
