use std::{env, path};
use std::collections::HashMap;
use std::sync::Mutex;

use fernet::Fernet;
use lazy_static::lazy_static;
use minijinja::Environment;

use crate::template;

pub fn get_binary() -> String {
    let binary = env::args().next().unwrap();
    path::Path::new(&binary).file_name().unwrap().to_str().unwrap().to_string()
}

#[derive(Debug)]
pub struct Cargo {
    pub binary: String,
    pub crate_name: String,
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

/// ## References
/// - https://doc.rust-lang.org/cargo/reference/environment-variables.html
/// - https://github.com/rust-lang/cargo/issues/8251#issuecomment-631731144
/// - https://github.com/rust-lang/cargo/issues/11966#issue-1664748892
pub fn build_info() -> Cargo {
    let cargo = Cargo {
        binary: get_binary(),
        crate_name: env!("CARGO_CRATE_NAME").to_string(),
        manifest_dir: env!("CARGO_MANIFEST_DIR").to_string(),
        authors: env!("CARGO_PKG_AUTHORS").split(',').map(String::from).collect(),
        description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        homepage: env!("CARGO_PKG_HOMEPAGE").to_string(),
        pkg_name: env!("CARGO_PKG_NAME").to_string(),
        pkg_repo: env!("CARGO_PKG_REPOSITORY").to_string(),
        pkg_version: env!("CARGO_PKG_VERSION").to_string(),
        pkg_version_major: env!("CARGO_PKG_VERSION_MAJOR").to_string(),
        pkg_version_minor: env!("CARGO_PKG_VERSION_MINOR").to_string(),
        pkg_version_patch: env!("CARGO_PKG_VERSION_PATCH").to_string(),
        pkg_version_pre: env!("CARGO_PKG_VERSION_PRE").to_string(),
    };
    cargo
}

lazy_static! {
    pub static ref FERNET: Fernet = Fernet::new(&generate_key()).unwrap();
}

/// Create a Fernet object to encrypt and decrypt session token.
///
/// ## References:
/// https://docs.rs/fernet/latest/fernet/
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
