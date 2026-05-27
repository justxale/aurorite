use crate::database::Overwrite;
use crate::database::{Character, Class, CreatureSize, CreatureType, Race};
use crate::responses::AuroriteErrorResponse;
use crate::utils::uuid::encode_uuid;
use serde::Serialize;

pub mod parts;
mod stats;

use parts::*;

#[derive(Debug, Serialize)]
pub struct CharacterInfo {
    id: String,
}

#[derive(Debug, Serialize)]
pub struct FullCharacterBaseInfo {
    id: String,
    level: u8,
    max_hits: u16,

    class: ClassInfo,
    background: BackgroundInfo,
    race: CharacterRaceInfo,
    abilities: CharacterAbilitiesInfo,
}

impl TryFrom<Character> for FullCharacterBaseInfo {
    type Error = AuroriteErrorResponse;
    fn try_from(char: Character) -> Result<Self, Self::Error> {
        if char.class.is_unloaded() || char.background.is_unloaded() || char.race.is_unloaded() {
            return Err(AuroriteErrorResponse::new("failed to collect data"));
        }
        let background = BackgroundInfo::from(char.background.get());
        let race = CharacterRaceInfo::from(char.race.get());
        let class = ClassInfo::from(char.class.get());
        let abilities = CharacterAbilitiesInfo::try_from(&char)?;
        Ok(Self {
            id: encode_uuid(&char.id),
            level: char.level,
            max_hits: char.max_hits,
            background,
            class,
            race,
            abilities,
        })
    }
}
