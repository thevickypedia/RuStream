use std::sync::Mutex;

use lazy_static::lazy_static;
use minijinja::Environment;

/// Create static objects that can be tossed around the modules
///
/// References:
///     https://doc.rust-lang.org/std/macro.include_str.html
pub static INDEX: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/index.html"));
pub static LANDING: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/land.html"));
pub static LISTING: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/list.html"));
// pub static LOGOUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/logout.html"));
pub static SESSION: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/session.html"));
pub static UNAUTHORIZED: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/unauthorized.html"));

lazy_static! {
    pub static ref ENV: Mutex<Environment<'static>> = Mutex::new({
        let mut env = Environment::new();
        env.add_template("index", INDEX).unwrap();
        env.add_template("landing", LANDING).unwrap();
        env.add_template("listing", LISTING).unwrap();
        env.add_template("session", SESSION).unwrap();
        env.add_template("unauthorized", UNAUTHORIZED).unwrap();
        env
    });
}
