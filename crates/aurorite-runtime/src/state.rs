use crate::{RuntimeEvent, Scene};
use crate::Character;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use aurorite_dataflow::database::{CampaignCharacter, Db};
use aurorite_dataflow::dto::SceneDto;

#[derive(Debug)]
pub struct Initiative {
    pub order: Vec<(Uuid, i64)>
}

impl Initiative {
    pub fn new() -> Self {
        Self {
            order: Vec::new()
        }
    }

    pub fn add_character(&mut self, character: &Character) {

    }
}

#[derive(Debug)]
pub struct RuntimeCtx {
    campaign_id: Uuid,
    scene: Option<Scene>,
    initiative: Option<Initiative>,
    characters: HashMap<Uuid, Character>,
    sender: Sender<RuntimeEvent>
}

impl RuntimeCtx {
    pub fn new(campaign_id: Uuid, sender: Sender<RuntimeEvent>) -> Self {
        Self {
            campaign_id, sender,
            initiative: None,
            scene: None,
            characters: HashMap::new(),
        }
    }

    pub fn switch_scene(&mut self, dto: SceneDto) {
        let mut characters = Vec::with_capacity(dto.preloads.len());
        for p in dto.preloads {
            characters.push((p.character.id.uuid(), p.is_visible));
            self.characters.insert(
                p.character.id.uuid(),
                Character::from(p.character)
            );
        }
        self.scene = Some(Scene {
            characters,
            asset: dto.asset,
        });
    }

    pub fn remove_scene(&mut self) {
        self.scene = None;
    }

    pub fn start_initiative(&mut self, character_ids: &[Uuid]) -> Result<(), &'static str> {
        let mut initiative = Initiative::new();
        for id in character_ids {
            match self.characters.get(id) {
                Some(c) => initiative.add_character(c),
                None => {
                    tracing::error!("character {} was not found", id);
                    return Err("character was not found")
                }
            }
        }

        self.initiative = Some(initiative);
        Ok(())
    }

    pub fn remove_initiative(&mut self) {
        self.initiative = None;
    }

    pub async fn save_state(&self, db: &mut Db) -> Result<(), &'static str> {
        let mut tx = db.transaction().await.map_err(|_| "db failure")?;
        for c in self.characters.values() {
            let _ = CampaignCharacter::update_by_character_id_and_campaign_id(c.id, self.campaign_id)
                .current_hits(c.current_hits)
                .exec(&mut tx).await;
        }
        tx.commit().await.map_err(|_| "transaction failed, data will not be saved")?;
        Ok(())
    }

    pub fn characters(&self) -> &HashMap<Uuid, Character> {
        &self.characters
    }

    pub fn character(&self, id: Uuid) -> Option<&Character> {
        self.characters.get(&id)
    }

    pub fn character_mut(&mut self, id: Uuid) -> Option<&mut Character> {
        self.characters.get_mut(&id)
    }
}