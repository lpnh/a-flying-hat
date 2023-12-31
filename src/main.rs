use axum::response::{IntoResponse, Json};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;

use a_flying_hat::templates::{click_response, index};

async fn ping() -> impl IntoResponse {
    Json("pong!")
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut app = Router::new()
        .route("/", get(index))
        .route("/clicked", post(click_response))
        .route("/ping", get(ping))
        .nest_service("/public", ServeDir::new("public"));

    if cfg!(debug_assertions) {
        info!("Enabling livereload");
        app = app.layer(tower_livereload::LiveReloadLayer::new());
    }

    let listener = tokio::net::TcpListener::bind("[::]:8080").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
