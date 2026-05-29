use uuid::Uuid;
use serde::Serialize;
use crate::database::{CreatureSize, CreatureType, Race};

impl CharacterRaceInfo {
    pub fn new(
        id: Uuid,
        l18n: String,
        size: CreatureSize,
        creature_type: CreatureType,
        speed: u16,
        dark_vision: Option<u16>,
        strength: u8,
        intelligence: u8,
        wisdom: u8,
        dexterity: u8,
        constitution: u8,
        charisma: u8,
    ) -> Self {
        Self {
            id,
            l18n,
            size,
            creature_type,
            speed,
            dark_vision,
            strength,
            intelligence,
            wisdom,
            dexterity,
            constitution,
            charisma,
        }
    }
}

impl From<&Race> for CharacterRaceInfo {
    fn from(race: &Race) -> Self {
        Self::new(
            race.id,
            race.l18n_key.clone(),
            race.size,
            race.creature_type,
            race.speed,
            race.dark_vision,
            race.strength,
            race.intelligence,
            race.wisdom,
            race.dexterity,
            race.constitution,
            race.charisma,
        )
    }
}

#[derive(Debug, Serialize)]
pub struct CharacterRaceInfo {
    id: Uuid,
    l18n: String,
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