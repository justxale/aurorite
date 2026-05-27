use crate::database::Campaign;
use crate::responses::{AuroriteErrorResponse, ClientInfo};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CampaignInfo {
    id: uuid::Uuid,
    title: String,
    masters: Vec<ClientInfo>,
    players: Vec<ClientInfo>,
}

impl TryFrom<&Campaign> for CampaignInfo {
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
