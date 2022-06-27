use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct DhtMeasurement {
    pub temperature: f64,
    pub humidity: f64,
}