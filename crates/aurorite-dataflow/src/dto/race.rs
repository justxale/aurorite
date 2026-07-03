use crate::database::{CreatureSize, CreatureType, Race};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceDto {
    id: Uuid,
    i18n: String,
    size: CreatureSize,
    #[serde(rename = "type")]
    creature_type: CreatureType,
    speed: u16,
    dark_vision: Option<u16>,

    strength: u8,
    intelligence: u8,
    wisdom: u8,
    dexterity: u8,
    constitution: u8,
    charisma: u8,
}

impl From<&Race> for RaceDto {
    fn from(race: &Race) -> Self {
        Self {
            id: race.id,
            i18n: race.i18n.clone(),
            size: race.size,
            creature_type: race.creature_type,
            speed: race.speed,
            dark_vision: race.dark_vision,
            strength: race.strength,
            intelligence: race.intelligence,
            wisdom: race.wisdom,
            dexterity: race.dexterity,
            constitution: race.constitution,
            charisma: race.charisma,
        }
    }
}
