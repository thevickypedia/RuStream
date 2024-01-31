use std::env;
use std::path::Path;

pub struct Server {
    pub workers: usize,
    pub max_connections: usize,
    pub port: usize,
}

fn getenv(key: &str, default: i32) -> i32 {
    let value = env::var(key);
    return match value {
        Ok(ok) => {
            ok.to_string().parse::<i32>().unwrap()
        }
        Err(_) => {
            default
        }
    };
}

impl Server {
    pub fn config() -> Server {
        Server {
            workers: getenv("workers", getenv("WORKERS", 3)) as usize,
            max_connections: getenv("max_connections", getenv("MAX_CONNECTIONS", 100)) as usize,
            port: getenv("port", getenv("PORT", 8000)) as usize
        }
    }
}

pub async fn get_binary() -> String {
    let binary = env::args().next().unwrap();
    Path::new(&binary).file_name().unwrap().to_str().unwrap().to_string()
}
