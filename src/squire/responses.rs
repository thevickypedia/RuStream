use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use minijinja::Template;

/// Frames a response for Not Found [404] into an error page.
///
/// # Arguments
///
/// * `error` - Jinja template for the error page.
/// * `description` - Description to be rendered in the UI.
///
/// # Returns
///
/// Returns an HTTPResponse with 404 error code formatted as HTML.
pub fn not_found(error: Template, description: &String, version: &String) -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body(error.render(minijinja::context!(
            version => version,
            title => "CONTENT UNAVAILABLE",
            description => description,
            help => r"Lost your way?\n\nHit the HOME button to navigate back to home page.",
            button_text => "HOME", button_link => "/home"
        )).unwrap())
}

/// Frames a response for Forbidden [403] into an error page.
///
/// # Arguments
///
/// * `error` - Jinja template for the error page.
/// * `username` - Username whose access is forbidden.
///
/// # Returns
///
/// Returns an HTTPResponse with 403 error code formatted as HTML.
pub fn restricted(error: Template, username: &String, version: &String) -> HttpResponse {
    HttpResponse::build(StatusCode::FORBIDDEN)
        .content_type("text/html; charset=utf-8")
        .body(error.render(minijinja::context!(
            version => version,
            title => "RESTRICTED SECTION",
            description => format!("This content is not accessible, as it does not belong to the user profile '{}'", username),
            help => r"Lost your way?\n\nHit the HOME button to navigate back to home page.",
            button_text => "HOME", button_link => "/home",
            block_navigation => true
        )).unwrap())
}
