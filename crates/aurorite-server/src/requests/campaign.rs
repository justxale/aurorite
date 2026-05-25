use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PostCampaign {
    pub title: String,
}