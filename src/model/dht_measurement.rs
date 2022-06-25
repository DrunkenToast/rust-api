use serde::Serialize;

#[derive(Serialize)]
pub struct DhtMeasurement {
    pub temperature: f64,
    pub humidity: f64,
}