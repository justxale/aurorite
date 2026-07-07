use crate::database::{AccessState, Campaign};
use crate::dto::client::ClientDto;
use crate::dto::SceneDto;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignDto {
    pub id: uuid::Uuid,
    pub title: String,
    pub access_state: AccessState,
    pub masters: Vec<ClientDto>,
    pub players: Vec<ClientDto>,
    pub scene: Option<SceneDto>,
    pub scenes: Vec<SceneDto>,

    pub last_played_at: Timestamp,
    pub created_at: Timestamp,
}

impl TryFrom<Campaign> for CampaignDto {
    type Error = &'static str;
    fn try_from(value: Campaign) -> Result<Self, Self::Error> {
        if value.clients.is_unloaded() {
            return Err("failed to fetch clients data");
        }
        if value.scene.is_unloaded() {
            return Err("failed to fetch scene data");
        }
        if value.scenes.is_unloaded() {
            return Err("failed to fetch scenes data");
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
        let scene: Option<SceneDto> = value
            .scene
            .get()
            .as_ref()
            .and_then(|v| SceneDto::try_from(v).ok());
        let scenes = value
            .scenes
            .get()
            .iter()
            .map(|v| SceneDto::try_from(v).unwrap())
            .collect();
        Ok(Self {
            id: value.id,
            title: value.title,
            access_state: value.access_state,
            masters,
            players,
            scene,
            scenes,
            last_played_at: value.last_played_at,
            created_at: value.created_at,
        })
    }
}
