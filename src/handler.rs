use axum::extract::State;

use crate::AppState;

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn get_data_from_db(State(state): State<AppState>) -> String {
    #[derive(Debug, Clone)]
    pub struct User {
        pub id: i64,
        pub name: String,
        pub email: String,
        pub address: Option<String>,
        pub created_at: chrono::NaiveDateTime,
    }
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
