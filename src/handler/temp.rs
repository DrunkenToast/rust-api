use axum::{
    routing::{get},
    Router, http::StatusCode, Json, response::IntoResponse, Extension
};

use crate::service::arduino::{ArduinoState};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(current_temp))
}

async fn current_temp(arduino: Extension<ArduinoState>) -> impl IntoResponse {
    //arduino.lock().await.display_message("Yippie!");
    (StatusCode::OK, Json("OK"))
}