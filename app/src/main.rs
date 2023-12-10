use axum::{
    extract,
    http::StatusCode,
    response::{self, Html},
    routing::{get, post},
    Router,
};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE as CONTENT_TYPE_REQWEST};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), StatusCode> {
    let app = Router::new()
        .route("/", get(handler_get))
        .route("/submit", post(push_message))
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

async fn push_message(
    extract::Json(body): extract::Json<Args>,
) -> Result<response::Json<Response>, StatusCode> {
    println!("POST/ {:?}", body);
    let url = "https://exp.host/--/api/v2/push/send";
    let _expo_token = "ExponentPushToken[TOWNJ5LmG02dzppStD58kK]";

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE_REQWEST,
        HeaderValue::from_static("application/json"),
    );

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
        .expect("Failed to send request");

    println!("{:?}", response.text().await.unwrap());
    module::expo_api::add();

    Ok(response::Json(Response { success: true }))
}
