use std::env;
use std::path::Path;

#[allow(dead_code)]
struct Server {
    workers: i32,
    max_connections: i32,
    port: i32,
}

pub async fn getenv(key: &str, default: i32) -> i32 {
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

pub async fn get_binary() -> String {
    let binary = env::args().next().unwrap();
    Path::new(&binary).file_name().unwrap().to_str().unwrap().to_string()
}
