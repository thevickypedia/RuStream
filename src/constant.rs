/// Create a Fernet object to encrypt and decrypt session token.
///
/// References:
///     https://docs.rs/fernet/latest/fernet/

use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

use fernet::Fernet;
use lazy_static::lazy_static;
use minijinja::{Environment};
use crate::template;
use crate::squire::startup::get_binary;

#[derive(Debug)]
pub struct Cargo {
    pub binary: String,
    pub manifest_dir: String,
    pub authors: Vec<String>,
    pub description: String,
    pub homepage: String,
    pub pkg_name: String,
    pub pkg_repo: String,
    pub pkg_version: String,
    pub pkg_version_major: String,
    pub pkg_version_minor: String,
    pub pkg_version_patch: String,
    pub pkg_version_pre: String,
}

pub fn build_info() -> Cargo {
    let cargo = Cargo {
        binary: get_binary(),
        manifest_dir: env::var("CARGO_MANIFEST_DIR").unwrap_or("NA".to_string()),
        authors: env::var("CARGO_PKG_AUTHORS").unwrap_or_default().split(',').map(String::from).collect(),
        description: env::var("CARGO_PKG_DESCRIPTION").unwrap_or("NA".to_string()),
        homepage: env::var("CARGO_PKG_HOMEPAGE").unwrap_or("NA".to_string()),
        pkg_name: env::var("CARGO_PKG_NAME").unwrap_or("NA".to_string()),
        pkg_repo: env::var("CARGO_PKG_REPOSITORY").unwrap_or("NA".to_string()),
        pkg_version: env::var("CARGO_PKG_VERSION").unwrap_or("NA".to_string()),
        pkg_version_major: env::var("CARGO_PKG_VERSION_MAJOR").unwrap_or("NA".to_string()),
        pkg_version_minor: env::var("CARGO_PKG_VERSION_MINOR").unwrap_or("NA".to_string()),
        pkg_version_patch: env::var("CARGO_PKG_VERSION_PATCH").unwrap_or("NA".to_string()),
        pkg_version_pre: env::var("CARGO_PKG_VERSION_PRE").unwrap_or("NA".to_string()),
    };
    cargo
}

lazy_static! {
    pub static ref FERNET: Fernet = Fernet::new(&generate_key()).unwrap();
}

fn generate_key() -> String {
    Fernet::generate_key()
}

lazy_static! {
    pub static ref ENV: Mutex<Environment<'static>> = Mutex::new({
        let mut env = Environment::new();
        env.add_template("index", template::INDEX).unwrap();
        env.add_template("landing", template::LANDING).unwrap();
        env.add_template("listing", template::LISTING).unwrap();
        env.add_template("logout", template::LOGOUT).unwrap();
        env.add_template("session", template::SESSION).unwrap();
        env.add_template("unauthorized", template::UNAUTHORIZED).unwrap();
        env
    });
}

lazy_static::lazy_static! {
    pub static ref HOST_SERVE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}
