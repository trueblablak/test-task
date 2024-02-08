use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Debug, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/search", post(search));

    axum::serve(TcpListener::bind("127.0.0.1:3000").await.unwrap(), router)
        .await
        .unwrap();
}

async fn login(Json(creds): Json<Credentials>) -> impl IntoResponse {
    "Hello!"
}

async fn register(Json(creds): Json<Credentials>) -> impl IntoResponse {
    "Hello!"
}

async fn search(Json(q): Json<String>) -> impl IntoResponse {
    "Hello!"
}
