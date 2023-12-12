use axum::{
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};

use tokio;
use tower_http::cors::{Any, CorsLayer};
mod module;

#[tokio::main]
async fn main() -> Result<(), StatusCode> {
    let app = Router::new()
        .route("/", get(handler_get))
        .route("/submit", post(module::expo_api::push_message))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn handler_get() -> Html<&'static str> {
    println!("GET / html");
    Html(include_str!("./static/index.html"))
}
