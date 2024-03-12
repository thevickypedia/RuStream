use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use minijinja::Template;

// todo: write docstrings

pub fn not_found(error: Template, description: &String) -> HttpResponse {
    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body(error.render(minijinja::context!(
            title => "CONTENT UNAVAILABLE",
            description => description,
            help => r"Lost your way?\n\nHit the HOME button to navigate back to home page.",
            button_text => "HOME", button_link => "/home"
        )).unwrap())
}

pub fn restricted(error: Template, username: &String) -> HttpResponse {
    HttpResponse::build(StatusCode::UNAUTHORIZED)
        .content_type("text/html; charset=utf-8")
        .body(error.render(minijinja::context!(
            title => "RESTRICTED SECTION",
            description => format!("This content is not accessible, as it does not belong to the user profile '{}'", username),
            help => r"Lost your way?\n\nHit the HOME button to navigate back to home page.",
            button_text => "HOME", button_link => "/home"
        )).unwrap())
}
