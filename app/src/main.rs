use dotenv;
use shuttle_secrets::SecretStore;
use sqlx::sqlite::SqlitePool;
use std::env;

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
    let pool = SqlitePool::connect(&state.database_url).await.unwrap();
    let users = sqlx::query_as!(
        User,
        "select id, name, email, address, created_at from users"
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for user in users.clone() {
        println!("{:?}", user);
    }

    users[0].name.clone()
}

#[derive(Debug, Clone)]
struct AppState {
    database_url: String,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    // dotenv::dotenv().expect("Failed to read .env file");
    let val = secret_store
        .get("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let state = AppState {
        database_url: val.clone(),
    };

    println!("🔥DATABASE_URL🔥: {}", val);

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/get", get(get_data_from_db))
        .with_state(state);

    Ok(router.into())
}

// #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {
//     dotenv::dotenv().expect("Failed to read .env file");
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let pool = SqlitePool::connect(&database_url).await?;
//     let users = sqlx::query_as!(
//         User,
//         "select id, name, email, address, created_at from users"
//     )
//     .fetch_all(&pool)
//     .await?;
//     for user in users {
//         println!("{:?}", user);
//     }

//     Ok(())
// }
