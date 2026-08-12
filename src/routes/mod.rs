/// Module for `/`, `/health` and `/profile` entrypoints.
pub mod basics;
/// Module for all the rendering based entry points.
pub mod media;
/// Module for `/home`, `/login` and `/logout` entrypoints.
pub mod auth;
/// Module for `/upload` entrypoint that handles the file uploads.
pub mod upload;
/// Module for `/edit` entrypoint that handles delete/rename actions.
pub mod fileio;

/// Joins a base URL prefix with a path, keeping the root prefix ("/") clean.
pub fn join_path(base_url: &str, path: &str) -> String {
    if base_url == "/" {
        path.to_string()
    } else {
        format!("{}{}", base_url, path)
    }
}
