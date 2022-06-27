use axum::{
    routing::{get},
    Json,
    Router,
    Extension,
};

use crate::{service::arduino::ArduinoState, model::{health::Health, response::ApiResponse}};

pub fn routes() -> Router {
    Router::new().route("/", get(health))
}

async fn health(Extension(arduino): Extension<ArduinoState>) -> ApiResponse<Json<Health>> {
    let res = tokio::spawn(async move {
        let mut arduino = arduino.lock().await;
        arduino.health().await
    }).await.unwrap()?;

    Ok(Json(Health {arduino: res}))
}