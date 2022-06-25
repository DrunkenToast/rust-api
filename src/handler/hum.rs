use axum::{
    routing::{get},
};

pub fn routes() -> Router {
    Router::new().route("/", get(&"TODO"))
}