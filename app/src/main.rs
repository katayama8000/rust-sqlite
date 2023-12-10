use axum::{
    extract,
    response::{self, Html},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // Build our application with routes for both GET and POST
    let app = Router::new()
        .route("/", get(handler_get))
        .route("/submit", post(ping));

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

#[derive(Deserialize, Debug)]
struct Args {
    expo_push_token: String,
    title: String,
    body: String,
}

#[derive(Serialize)]
struct Response {
    success: bool,
}

async fn ping(extract::Json(body): extract::Json<Args>) -> response::Json<Response> {
    println!("{:?}", body);
    response::Json(Response { success: true })
}
