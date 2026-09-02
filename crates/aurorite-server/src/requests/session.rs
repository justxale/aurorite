use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PostSessionMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct PostSessionInitiative {
    pub members: Vec<Uuid>
}
