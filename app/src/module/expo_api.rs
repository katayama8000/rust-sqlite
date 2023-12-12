use axum::{
    extract,
    http::StatusCode,
    response::{self},
};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct Args {
    expo_push_token: String,
    title: String,
    body: String,
}

#[derive(Serialize)]
pub struct Response {
    success: bool,
}

pub async fn push_message(
    extract::Json(body): extract::Json<Args>,
) -> Result<response::Json<Response>, StatusCode> {
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
        .expect("Failed to send request");

    println!("{:?}", response.text().await.unwrap());

    Ok(response::Json(Response { success: true }))
}
