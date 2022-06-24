use axum::{
    extract,
    routing::{get, patch},
    Json,
    Router,
    response::IntoResponse,
    http::StatusCode, Extension,
};
use crate::{arduino::ArduinoState, model::led::PutLedData};

pub fn routes() -> Router {
    Router::new().route("/", patch(switch_led))
}

async fn switch_led(extract::Json(input): extract::Json<PutLedData>, Extension(arduino): Extension<ArduinoState>) -> impl IntoResponse {
    (StatusCode::OK, Json("OK"))
}