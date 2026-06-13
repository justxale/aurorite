use serde::{Deserialize, Serialize};
use crate::database::Campaign;
use crate::dto::client::ClientDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignDto {
    pub id: uuid::Uuid,
    pub title: String,
    pub masters: Vec<ClientDto>,
    pub players: Vec<ClientDto>,
}

impl TryFrom<&Campaign> for CampaignDto {
    type Error = &'static str;
    fn try_from(value: &Campaign) -> Result<Self, Self::Error> {
        if value.clients.is_unloaded() {
            return Err("failed to fetch clients");
        }
        let masters: Vec<ClientDto> = value
            .clients
            .get()
            .iter()
            .filter(|cl| cl.is_master)
            .map(|cl| ClientDto::from(cl.client.get()))
            .collect();
        let players = value
            .clients
            .get()
            .iter()
            .filter(|cl| !cl.is_master)
            .map(|cl| ClientDto::from(cl.client.get()))
            .collect();

        Ok(Self {
            masters,
            players,
            id: value.id,
            title: value.title.clone(),
        })
    }
}