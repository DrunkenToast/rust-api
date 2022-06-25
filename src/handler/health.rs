use axum::{
    routing::{get},
    Json,
    Router,
    response::IntoResponse,
    http::StatusCode, Extension,
};

use crate::{service::arduino::ArduinoState, model::health::Health};

pub fn routes() -> Router {
    Router::new().route("/", get(health))
}

async fn health(Extension(arduino): Extension<ArduinoState>) -> impl IntoResponse {
    let h = Health {
        arduino: arduino.lock().await.health().await
    };
    (StatusCode::OK, Json(h))
}