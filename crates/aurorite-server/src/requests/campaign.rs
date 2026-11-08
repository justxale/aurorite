use aurorite_dataflow::database::Visibility;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PostCampaign {
    pub title: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct PutCampaignSession {
    #[schema(value_type = String)]
    pub visibility: Visibility,
}
