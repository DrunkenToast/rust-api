
use axum::{
    routing::{get},
    Router, Extension, Json
};

use crate::{service::{arduino::{ArduinoState}, sql::DatabaseState}, model::{response::ApiResponse, dht_measurement::DhtMeasurement, error::DatabaseError}};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(dht_measurement))
        .route("/history", get(dht_history))
}

async fn dht_measurement(arduino: Extension<ArduinoState>) -> ApiResponse<Json<DhtMeasurement>> {
    let res = tokio::spawn(async move {
        let mut arduino = arduino.lock().await;
        arduino.measure_dht().await
    }).await.unwrap()?;

    Ok(Json(res))
}

async fn dht_history(db: Extension<DatabaseState>) -> ApiResponse<Json<Vec<DhtMeasurement>>> {
    let db = db.lock().await;
    let res = match DhtMeasurement::select_all(&db) {
        Ok(val) => Ok(val),
        Err(_) => Err(DatabaseError)
    };
    Ok(Json(res?))
}