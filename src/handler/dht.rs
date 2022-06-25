
use axum::{
    routing::{get},
    Router, Extension, Json
};

use crate::{service::arduino::{ArduinoState}, model::{response::ApiResponse, dht_measurement::DhtMeasurement}};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(dht_measurement))
}

async fn dht_measurement(arduino: Extension<ArduinoState>) -> ApiResponse<Json<DhtMeasurement>> {
    let mut arduino = arduino.lock().await;
    Ok(Json(arduino.measure_dht().await?))
}