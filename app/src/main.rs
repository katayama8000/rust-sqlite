use axum::{
    extract,
    response::{self, Html},
    routing::{get, post},
    Router,
};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() {
    // Build our application with routes for both GET and POST
    let app = Router::new()
        .route("/", get(handler_get))
        .route("/submit", post(push_message));

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

async fn push_message(extract::Json(body): extract::Json<Args>) -> response::Json<Response> {
    println!("{:?}", body);
    let url = "https://exp.host/--/api/v2/push/send";
    let _expo_token = "ExponentPushToken[TOWNJ5LmG02dzppStD58kK]";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    let payload = json!({
        "to": body.expo_push_token,
        "title":body.title,
        "body": body.body,
    });

    let response = client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
        .unwrap();

    println!("{:?}", response.text().await.unwrap());

    response::Json(Response { success: true })
}
