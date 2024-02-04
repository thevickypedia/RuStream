/// Create a Fernet object to encrypt and decrypt session token.
///
/// References:
///     https://docs.rs/fernet/latest/fernet/

use std::collections::HashMap;
use std::sync::Mutex;

use fernet::Fernet;
use lazy_static::lazy_static;
use minijinja::{Environment};
use crate::template;

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
