use axum::{
    body::Body,
    response::Html,
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    // Build our application with routes for both GET and POST
    let app = Router::new()
        .route("/", get(handler_get))
        .route("/submit", post(handler_post));

    // Run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler_get() -> Html<&'static str> {
    Html(include_str!("./static/index.html"))
}

async fn handler_post(body: Body) -> &'static str {
    "Hello, World! (POST)"
}
