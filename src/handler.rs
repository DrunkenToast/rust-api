use axum::{response::IntoResponse, http::StatusCode};

pub mod health;
pub mod led;
pub mod dht;

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Imagine a tumbleweed crossing your screen")
}