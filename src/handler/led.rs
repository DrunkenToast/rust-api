use axum::{
    extract,
    routing::{put},
    Json,
    Router,
    response::IntoResponse,
    http::StatusCode, Extension,
};
use crate::{service::arduino::{ArduinoState, ArduinoError}, model::led::PutLedData};

pub fn routes() -> Router {
    Router::new().route("/", put(switch_led))
}

async fn switch_led(extract::Json(input): extract::Json<PutLedData>, Extension(arduino): Extension<ArduinoState>) -> impl IntoResponse {
    match arduino.lock().await.switch_led(input.state).await {
        Ok(_) =>  {
            (StatusCode::OK, Json("OK"))
        },
        Err(ArduinoError::Timeout) => {
            (StatusCode::GATEWAY_TIMEOUT, Json("Arduino response timed out"))
        },
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error"))
        }
    }
}