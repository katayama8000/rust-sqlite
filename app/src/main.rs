use dotenv;
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

use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn get_data_from_db() -> String {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await.unwrap();
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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenv::dotenv().expect("Failed to read .env file");

    let router = Router::new()
        .route("/", get(hello_world))
        .route("/get", get(get_data_from_db));

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
