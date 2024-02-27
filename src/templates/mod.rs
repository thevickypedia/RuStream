use std::sync::Arc;

/// Index page template that is served as HTML response for the root endpoint.
mod index;
/// Landing page template that is served as HTML response while streaming media.
mod landing;
/// Listing page template that is served as HTML response after successful authentication.
mod listing;
/// Logout page template that is served as HTML response when the user decides to end the session.
mod logout;
/// Session page template that is served as HTML response when invalid/expired session tokens are received.
mod session;
/// Unauthorized page template that is served as HTML response after failed authentication.
mod unauthorized;

/// Loads all the HTML templates' content into a Jinja Environment
///
/// # Returns
///
/// Returns the constructed `Arc` for the `Environment` object, that holds the central configuration state for templates.
/// It is also the container for all loaded templates.
pub fn environment() -> Arc<minijinja::Environment<'static>> {
    let mut env = minijinja::Environment::new();
    env.add_template_owned("index", index::get_content()).unwrap();
    env.add_template_owned("landing", landing::get_content()).unwrap();
    env.add_template_owned("listing", listing::get_content()).unwrap();
    env.add_template_owned("logout", logout::get_content()).unwrap();
    env.add_template_owned("session", session::get_content()).unwrap();
    env.add_template_owned("unauthorized", unauthorized::get_content()).unwrap();
    Arc::new(env)
}
