use std::path::Path;
use std::sync::{Arc, Mutex};

/// Read the content of each file and return it as a String.
///
/// # Arguments
///
/// * `filename` - Filename that has to be read.
///
/// # Returns
///
/// String representation of the file content.
pub fn get_content(filename: &str) -> String {
    let filepath = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("templates")
        .join(format!("{}.html", filename))
        .to_string_lossy()
        .to_string();
    std::fs::read_to_string(&filepath).unwrap_or_else(|_| String::new())
}

/// Reads all the HTML files in templates directory and loads the content into a Jinja Environment
///
/// # Rendered files
/// - Index page template that is served as HTML response for the root endpoint.
/// - Landing page template that is served as HTML response while streaming videos.
/// - Listing page template that is served as HTML response after successful authentication.
/// - Logout page template that is served as HTML response when the user decides to end the session.
/// - Session page template that is served as HTML response when invalid/expired session tokens are received.
/// - Unauthorized page template that is served as HTML response after failed authentication.
pub fn environment() -> Arc<Mutex<minijinja::Environment<'static>>> {
    let mut env = minijinja::Environment::new();
    for html in ["landing", "listing", "logout", "session"] {
        let content = get_content(&html);
        env.add_template_owned(html, content).unwrap();
    }
    let mutex = Mutex::new(env.to_owned());
    Arc::new(mutex)
}
