use aurorite_dataflow::database::{Campaign};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignInfo {
    pub id: uuid::Uuid,
    pub title: String,
}

impl From<&Campaign> for CampaignInfo {
    fn from(value: &Campaign) -> Self {
        Self {
            id: value.id,
            title: value.title.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCampaigns {
    pub campaigns: Vec<CampaignInfo>,
}

impl From<&Vec<Campaign>> for ClientCampaigns {
    fn from(campaigns: &Vec<Campaign>) -> Self {
        Self {
            campaigns: campaigns.iter().map(|c| c.into()).collect(),
        }
    }
}
