use handler::{get_data_from_db, hello_world};
use shuttle_secrets::SecretStore;
use sqlx::sqlite::SqlitePool;

mod domain;
mod handler;
mod usecase;

use axum::{routing::get, Router};

#[derive(Debug, Clone)]
struct AppState {
    pool: SqlitePool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let database_url = secret_store
        .get("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let state = AppState {
        pool: SqlitePool::connect(&database_url).await.unwrap(),
    };

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/get", get(get_data_from_db))
        .with_state(state);

    Ok(router.into())
}
