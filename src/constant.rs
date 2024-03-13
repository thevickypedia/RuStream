use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use fernet::Fernet;

pub static IMAGE_FORMATS: [&str; 7] = ["jpeg", "jpg", "png", "gif", "bmp", "svg", "webp"];
pub static SECURE_INDEX: &str = "SECURE_INDEX";

/// Struct to store the cargo information gathered at compile time using the `env!` macro.
#[derive(Debug, Clone)]
pub struct MetaData {
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
pub fn build_info() -> Arc<MetaData> {
    let metadata = MetaData {
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
    Arc::new(metadata)
}

/// Struct to store the session information.
///
/// ## Fields
///
/// * `tracker` - Used to log connection and streaming information without redundancy.
/// * `mapping` - Used to store username and session token's payload as key value pairs.
///
/// ## See Also:
///
/// These fields are updated and used only for authenticated sessions.
pub struct Session {
    pub tracker: Mutex<HashMap<String, String>>,
    pub mapping: Mutex<HashMap<String, String>>,
}


/// Instantiates the `Session` struct with empty `HashMap` for both `tracker` and `mapping` fields.
///
/// ## See Also
///
/// Creates new `Mutex` in an unlocked state for each of the fields.
///
/// # Returns
///
/// Returns the constructed `Arc` for the `Session` struct.
pub fn session_info() -> Arc<Session> {
    Arc::new(Session {
        tracker: Mutex::new(HashMap::new()),
        mapping: Mutex::new(HashMap::new()),
    })
}

/// Create a [Fernet](https://docs.rs/fernet/latest/fernet/) object to encrypt and decrypt session token.
///
/// Generates a random key, that can be safely passed to `Fernet::new()`
///
/// # Returns
///
/// Returns the constructed `Arc` for the `Fernet` instance, with the generated key.
pub fn fernet_object() -> Arc<Fernet> {
    Arc::new(Fernet::new(&Fernet::generate_key()).unwrap())
}
