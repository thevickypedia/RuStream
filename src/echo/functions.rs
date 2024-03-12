use chrono::{Local, Utc};
use crate::echo::display;

fn get_time(utc: bool) -> String {
    if utc {
        Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    } else {
        Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    }
}

#[allow(dead_code)]
pub fn debug_func(msg: &str, utc: bool) {
    let colors = display::colors();
    println!("[{} {}DEBUG{} {}] {}", get_time(utc), colors.light_green, colors.end, env!("CARGO_CRATE_NAME"), msg)
}

pub fn info_func(msg: &str, utc: bool) {
    let colors = display::colors();
    println!("[{} {}INFO{} {}] {}", get_time(utc), colors.green, colors.end, env!("CARGO_CRATE_NAME"), msg)
}

#[allow(dead_code)]
pub fn warn_func(msg: &str, utc: bool) {
    let colors = display::colors();
    println!("[{} {}WARN{} {}] {}", get_time(utc), colors.yellow, colors.end, env!("CARGO_CRATE_NAME"), msg)
}

#[allow(dead_code)]
pub fn error_func(msg: &str, utc: bool) {
    let colors = display::colors();
    println!("[{} {}ERROR{} {}] {}", get_time(utc), colors.red, colors.end, env!("CARGO_CRATE_NAME"), msg)
}

#[allow(dead_code)]
pub fn critical_func(msg: &str, utc: bool) {
    let colors = display::colors();
    let format = display::format();
    println!("[{} {}{}CRITICAL{} {}] {}", get_time(utc), format.bold, colors.red, colors.end, env!("CARGO_CRATE_NAME"), msg)
}
