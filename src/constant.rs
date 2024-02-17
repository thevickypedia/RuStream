use std::env;
use std::collections::HashMap;
use std::sync::Mutex;

use fernet::Fernet;
use lazy_static::lazy_static;

/// Struct to store the cargo information gathered at compile time using the `env!` macro.
#[derive(Debug)]
pub struct Cargo {
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

/// Uses compile time macros to load Cargo metadata via environment variables during compilation process
///
/// ## References
/// - [Official Docs](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
/// - [GitHub Issues](https://github.com/rust-lang/cargo/issues/8251#issuecomment-631731144)
/// - [GitHub Issues](https://github.com/rust-lang/cargo/issues/11966#issue-1664748892)
pub fn build_info() -> Cargo {
    let cargo = Cargo {
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
    pub static ref FERNET: Fernet = Fernet::new(&fernet_key()).unwrap();
}

/// Create a [Fernet](https://docs.rs/fernet/latest/fernet/) object to encrypt and decrypt session token.
///
/// Generates a random key, that can be safely passed to `Fernet::new()`
///
/// # Returns
///
/// Random key as a `String`
fn fernet_key() -> String {
    Fernet::generate_key()
}

lazy_static::lazy_static! {
    pub static ref HOST_SERVE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}
