/// Index page template that is served as HTML response for the root endpoint.
pub mod index;
/// Landing page template that is served as HTML response while streaming videos.
pub mod landing;
/// Listing page template that is served as HTML response after successful authentication.
pub mod listing;
/// Logout page template that is served as HTML response when the user decides to end the session.
pub mod logout;
/// Session page template that is served as HTML response when invalid/expired session tokens are received.
pub mod session;
/// Unauthorized page template that is served as HTML response after failed authentication.
pub mod unauthorized;
