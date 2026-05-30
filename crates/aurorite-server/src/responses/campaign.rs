use crate::database::Campaign;
use crate::responses::{AuroriteErrorResponse, ClientInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullCampaignInfo {
    pub id: uuid::Uuid,
    pub title: String,
    pub masters: Vec<ClientInfo>,
    pub players: Vec<ClientInfo>,
}

impl TryFrom<&Campaign> for FullCampaignInfo {
    type Error = AuroriteErrorResponse;
    fn try_from(value: &Campaign) -> Result<Self, Self::Error> {
        if value.clients.is_unloaded() {
            return Err(AuroriteErrorResponse::new("failed to fetch clients"));
        }
        let masters: Vec<ClientInfo> = value
            .clients
            .get()
            .iter()
            .filter(|cl| cl.is_master)
            .map(|cl| ClientInfo::from(cl.client.get()))
            .collect();
        let players = value
            .clients
            .get()
            .iter()
            .filter(|cl| !cl.is_master)
            .map(|cl| ClientInfo::from(cl.client.get()))
            .collect();

        Ok(Self {
            masters,
            players,
            id: value.id,
            title: value.title.clone(),
        })
    }
}

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
