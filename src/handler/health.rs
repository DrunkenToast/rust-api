use axum::{
    routing::{get},
    Json,
    Router,
    response::IntoResponse,
    http::StatusCode,
};

pub fn routes() -> Router {
    Router::new().route("/", get(health))
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json("OK"))
}