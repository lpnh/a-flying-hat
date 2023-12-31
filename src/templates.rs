use askama::Template;
use axum::response::IntoResponse; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "index.html")] // using the template in this path, relative to the `templates` dir in the crate root
pub struct IndexTemplate<'a> {
    home_h1: &'a str, // the field name should match the variable name
    home_h3: &'a str,               // in your template
}

pub async fn index() -> impl IntoResponse {
    // instantiate your struct
    IndexTemplate { 
        home_h1: "a flying hat",
        home_h3: "My Web App template leveraging axum, Askama, htmx and Tailwind. Deployed with Fly.io. "
    }
}

#[derive(Template)]
#[template(path = "click_response.html")]
pub struct ClickResponseTemplate {}

pub async fn click_response() -> impl IntoResponse {
    ClickResponseTemplate {}
}
