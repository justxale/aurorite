use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostSessionMessage {
    pub content: String,
}
