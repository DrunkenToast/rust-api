use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostMsgData {
    pub message: String,
}

