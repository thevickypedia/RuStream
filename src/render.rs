/// Create static objects that can be tossed around the modules
///
/// References:
///     https://doc.rust-lang.org/std/macro.include_str.html
pub static INDEX: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/index.html"));
pub static LANDING: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/land.html"));
pub static LISTING: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/list.html"));
// pub static LOGOUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/logout.html"));
pub static SESSION: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/session.html"));
pub static UNAUTHORIZED: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/unauthorized.html"));
