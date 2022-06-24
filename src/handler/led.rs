use axum::{
    extract,
    routing::{get, patch, post, put},
    Json,
    Router,
    response::IntoResponse,
    http::StatusCode, Extension,
};
use crate::{arduino::ArduinoState, model::led::PutLedData};

pub fn routes() -> Router {
    Router::new().route("/", put(switch_led))
}

async fn switch_led(extract::Json(input): extract::Json<PutLedData>, Extension(arduino): Extension<ArduinoState>) -> impl IntoResponse {
    arduino.lock().await.switch_led(input.state);
    (StatusCode::OK, Json("OK"))
}