use axum::{
    extract,
    routing::{put},
    Router,
    Extension,
};
use crate::{service::arduino::{ArduinoState}, model::{led::PutLedData, response::ApiResponse}};

pub fn routes() -> Router {
    Router::new().route("/", put(switch_led))
}

async fn switch_led(extract::Json(input): extract::Json<PutLedData>, Extension(arduino): Extension<ArduinoState>) -> ApiResponse<()> {
    tokio::spawn(async move {
        let mut arduino = arduino.lock().await;
        arduino.switch_led(input.state).await
    }).await.unwrap()?;

    Ok(())
}