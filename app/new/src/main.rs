use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn get_data_from_db() -> &'static str {
    "Hello, world!!!!!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/get", get(get_data_from_db));

    Ok(router.into())
}
