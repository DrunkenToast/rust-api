use axum::{
    extract,
    routing::{post},
    Router,
    Extension,
};
use crate::{service::arduino::{ArduinoState}, model::{response::ApiResponse, msg::PostMsgData}};

pub fn routes() -> Router {
    Router::new().route("/", post(post_msg))
}

async fn post_msg(extract::Json(input): extract::Json<PostMsgData>, Extension(arduino): Extension<ArduinoState>) -> ApiResponse<()> {
    tokio::spawn(async move {
        let mut arduino = arduino.lock().await;
        arduino.display_message(&input.message).await
    }).await.unwrap()?;

    Ok(())
}