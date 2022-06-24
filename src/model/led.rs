use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PutLedData {
    pub state: bool,
}
