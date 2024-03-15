/// Module for `/`, `/health` and `/profile` entrypoints.
pub mod basics;
/// Module for all the rendering based entry points.
pub mod media;
/// Module for `/home`, `/login`, `/logout` and `/error` entrypoints.
pub mod auth;
/// Module for `/upload` entrypoint that handles the file uploads.
pub mod upload;
/// Module for `/edit` entrypoint that handles delete/rename actions.
pub mod fileio;
