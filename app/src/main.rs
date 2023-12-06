use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://exp.host/--/api/v2/push/send";
    let expo_token = "ExponentPushToken[TOWNJ5LmG02dzppStD58kK]";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();

    let payload = json!({
        "to": expo_token,
        "title": "hello",
        "body": "rust",
    });

    let response = client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    println!("{:?}", response.text().await?);

    Ok(())
}
