use crate::Character;
use crate::{RuntimeEvent, Scene};
use aurorite_dataflow::dto::SceneDto;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use aurorite_dataflow::enums::Ability;
use crate::events::{InitiativeOrder, Throw};

#[derive(Debug)]
pub struct Initiative {
    pub order: Vec<(Uuid, i64)>,
}

impl Initiative {
    pub fn new() -> Self {
        Self { order: Vec::new() }
    }

    pub fn add_character(&mut self, character: &Character) -> (Uuid, i64, i16) {
        let dice = character.ability_dice(Ability::Dexterity);
        let res = dice.roll();
        self.order.push((character.id, res.sum));
        (character.id, res.sum, dice.bonus.unwrap_or(0))
    }

    pub fn add_characters(&mut self, characters: &[&Character]) -> Vec<(Uuid, i64, i16)> {
        let mut rolls: Vec<(Uuid, i64, i16)> = Vec::with_capacity(characters.len());
        self.order.reserve(characters.len());
        for c in characters {
            let res = self.add_character(c);
            rolls.push(res);
        }
        rolls
    }

    pub fn finalize(&mut self) -> &Vec<(Uuid, i64)> {
        self.order.sort_by_key(|entry| entry.1);
        self.order.reverse();
        &self.order
    }
}

#[derive(Debug)]
pub struct RuntimeCtx {
    campaign_id: Uuid,
    scene: Option<Scene>,
    initiative: Option<Initiative>,
    characters: HashMap<Uuid, Character>,
    sender: Sender<RuntimeEvent>,
}

impl RuntimeCtx {
    pub fn new(campaign_id: Uuid, sender: Sender<RuntimeEvent>) -> Self {
        Self {
            campaign_id,
            sender,
            initiative: None,
            scene: None,
            characters: HashMap::new(),
        }
    }

    pub fn switch_scene(&mut self, dto: SceneDto) {
        let mut characters = Vec::with_capacity(dto.preloads.len());
        for p in dto.preloads {
            characters.push((p.character.id.uuid(), p.is_visible));
            self.characters
                .insert(p.character.id.uuid(), Character::from(p.character));
        }
        self.scene = Some(Scene {
            characters,
            asset: dto.asset,
        });
    }

    pub fn remove_scene(&mut self) {
        self.scene = None;
    }

    pub async fn start_initiative(&mut self, character_ids: &[Uuid]) -> Result<(), &'static str> {
        let mut initiative = Initiative::new();
        let mut characters: Vec<&Character> = Vec::with_capacity(character_ids.len());
        for id in character_ids {
            match self.characters.get(id) {
                Some(c) => characters.push(c),
                None => {
                    tracing::error!("character {} was not found", id);
                    return Err("character was not found");
                }
            }
        }
        let throws = initiative.add_characters(&characters);
        let throws = throws.iter().copied().map(|(source, res, bonus)| Throw { source, res, bonus, all: None }).collect();
        let _ = self.sender.send(
            RuntimeEvent::ThrowDices(throws)
        ).await;
        let order =initiative.finalize();
        let _ = self.sender.send(RuntimeEvent::FinalizeInitiative(
            order.iter().copied().map(|(target, value)| { InitiativeOrder { target, value } }).collect()
        )).await;


        self.initiative = Some(initiative);
        Ok(())
    }

    pub fn remove_initiative(&mut self) {
        self.initiative = None;
    }

    pub fn characters_current_hits(&self) -> Vec<(Uuid, u16)> {
        self.characters
            .values()
            .map(|c| (c.id, c.current_hits))
            .collect()
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
