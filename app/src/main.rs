use axum::{
    extract::Path,
    routing::get,
    Json, Router,
};
use lambda_http::{run, Error};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
}

// GET /
async fn hello() -> &'static str {
    "Hello from Rust Lambda!"
}

// GET /health
async fn health() -> Json<Message> {
    Json(Message {
        message: "OK".to_string(),
    })
}

// GET /users/:id
async fn get_user(Path(id): Path<String>) -> Json<User> {
    Json(User {
        id,
        name: "Sample User".to_string(),
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // ルーター構築
    let app = Router::new()
        .route("/", get(hello))
        .route("/health", get(health))
        .route("/users/:id", get(get_user));

    // Lambda環境で実行（これだけでLambda化！）
    run(app).await
}