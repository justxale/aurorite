use serde::Deserialize;
use crate::enums::Skill;
use crate::utils::uuid::EncodedUuid;

#[derive(Deserialize)]
pub struct PostCharacterBase {
    pub level: u8,

    pub name: Option<String>,
    pub full_name: String,

    pub strength: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub charisma: u8,
}

#[derive(Debug, Deserialize)]
pub struct PutCharacterClass {
    pub class_id: EncodedUuid,
    pub chosen_skills: Vec<Skill>
}

#[derive(Debug, Deserialize)]
pub struct PutCharacterRace {
    pub race_id: EncodedUuid,
}

#[derive(Debug, Deserialize)]
pub struct PutCharacterBackground {
    pub background_id: EncodedUuid,
}