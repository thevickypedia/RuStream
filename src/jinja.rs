use std::sync::{Arc, Mutex};

use crate::templates;

/// Loads all the HTML content into a Jinja Environment
pub fn environment() -> Arc<Mutex<minijinja::Environment<'static>>> {
    let mut env = minijinja::Environment::new();
    env.add_template_owned("index", templates::index::get_content()).unwrap();
    env.add_template_owned("landing", templates::landing::get_content()).unwrap();
    env.add_template_owned("listing", templates::listing::get_content()).unwrap();
    env.add_template_owned("logout", templates::logout::get_content()).unwrap();
    env.add_template_owned("session", templates::session::get_content()).unwrap();
    env.add_template_owned("unauthorized", templates::unauthorized::get_content()).unwrap();
    let mutex = Mutex::new(env.to_owned());
    Arc::new(mutex)
}
