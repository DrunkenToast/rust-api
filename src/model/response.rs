use axum::{http::StatusCode, Json};
use serde_json::{Value, json};

use super::error::ArduinoError;


pub type ApiError = (StatusCode, Json<Value>);
pub type ApiResponse<T> = std::result::Result<T, ApiError>;

impl From<ArduinoError> for ApiError {
    fn from(err: ArduinoError) -> Self {
        match err {
            ArduinoError::IoError => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                    "message": err.to_string() 
                })))
            },
            ArduinoError::Timeout => {
                (StatusCode::GATEWAY_TIMEOUT, Json(json!({
                    "message": err.to_string()
                })))
            }
        }
    }
}