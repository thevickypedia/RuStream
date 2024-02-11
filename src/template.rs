/// Index page template that is served as HTML response for the root endpoint.
pub static INDEX: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/index.html"));

/// Landing page template that is served as HTML response while streaming videos.
pub static LANDING: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/land.html"));

/// Listing page template that is served as HTML response after successful authentication.
pub static LISTING: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/list.html"));

/// Logout page template that is served as HTML response when the user decides to end the session.
pub static LOGOUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/logout.html"));

/// Session page template that is served as HTML response when invalid/expired session tokens are received.
pub static SESSION: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/session.html"));

/// Unauthorized page template that is served as HTML response after failed authentication.
pub static UNAUTHORIZED: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/unauthorized.html"));
