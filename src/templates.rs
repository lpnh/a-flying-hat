use askama::Template;
use axum::response::IntoResponse; // bring trait in scope

#[derive(Template)] // this will generate the code...
#[template(path = "home.html")] // using the template in this path, relative to the `templates` dir in the crate root
pub struct HomeTemplate<'a> {
    page_title: &'a str, // the field name should match the variable name
    greetings: &'a str,  // in your template
}

pub async fn index() -> impl IntoResponse {
    // instantiate your struct
    HomeTemplate {
        page_title: "Home",
        greetings: "Hello, friend!",
    }
}

#[derive(Template)]
#[template(path = "click_response.html")]
pub struct ClickResponseTemplate {}

pub async fn click_response() -> impl IntoResponse {
    ClickResponseTemplate {}
}
