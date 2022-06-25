use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    pub arduino: HealthStatus, 
}

#[derive(Serialize)]
pub enum HealthStatus {
    Up,
    Down,
}