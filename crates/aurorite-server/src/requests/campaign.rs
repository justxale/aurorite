use serde::Deserialize;
use aurorite_dataflow::database::Visibility;

#[derive(Debug, Clone, Deserialize)]
pub struct PostCampaign {
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PutCampaignSession {
    pub visibility: Visibility
}