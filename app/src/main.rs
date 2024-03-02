use shuttle_secrets::SecretStore;
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub address: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

use axum::{extract::State, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn get_data_from_db(State(state): State<AppState>) -> String {
    let users = sqlx::query_as!(
        User,
        "select id, name, email, address, created_at from users"
    )
    .fetch_all(&state.pool)
    .await
    .unwrap();
    for user in users.clone() {
        println!("{:?}", user);
    }

    users[0].name.clone()
}

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
