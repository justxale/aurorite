use serde::Deserialize;
use aurorite_util::uuid::EncodedUuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct PostSessionMessage {
    pub content: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PostSessionInitiative {
    #[schema(value_type = Vec<String>)]
    pub members: Vec<EncodedUuid>
}
