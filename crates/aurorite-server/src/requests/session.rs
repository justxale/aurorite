use serde::Deserialize;
use aurorite_util::uuid::EncodedUuid;

#[derive(Debug, Deserialize)]
pub struct PostSessionMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct PostSessionInitiative {
    pub members: Vec<EncodedUuid>
}
