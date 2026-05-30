use crate::database::Character;
use crate::responses::AuroriteErrorResponse;
use crate::utils::uuid::EncodedUuid;
use serde::Serialize;

mod parts;
mod stats;

use parts::*;
use crate::responses::background::BackgroundInfo;
use crate::responses::class::ClassInfo;
use crate::responses::race::RaceInfo;

#[derive(Debug, Serialize)]
pub struct CharacterInfo {
    id: EncodedUuid,
    level: u8,
    name: Option<String>,
    full_name: String,
    class_l18n: Option<String>,
    background_l18n: Option<String>,
    race_l18n: Option<String>,
}

impl TryFrom<&Character> for CharacterInfo {
    type Error = AuroriteErrorResponse;
    fn try_from(character: &Character) -> Result<Self, Self::Error> {
        if character.class.is_unloaded() || character.background.is_unloaded() || character.race.is_unloaded() {
            return Err(AuroriteErrorResponse::new("failed to collect data"));
        }
        let class_l18n = character.class.get().as_ref().map(|data| &data.l18n_key);
        let background_l18n = character.background.get().as_ref().map(|data| &data.l18n_key);
        let race_l18n = character.race.get().as_ref().map(|data| &data.l18n_key);
        Ok(Self {
            id: EncodedUuid(character.id), level: character.level,
            name: character.name.clone(), full_name: character.full_name.clone(),
            class_l18n: class_l18n.cloned(), background_l18n: background_l18n.cloned(),
            race_l18n: race_l18n.cloned()
        })
    }
}

#[derive(Debug, Serialize)]
pub struct FullCharacterBaseInfo {
    id: EncodedUuid,
    level: u8,
    max_hits_overwrite: Option<u16>,

    class: Option<ClassInfo>,
    background: Option<BackgroundInfo>,
    race: Option<RaceInfo>,
    abilities: CharacterAbilitiesInfo,
}

impl TryFrom<&Character> for FullCharacterBaseInfo {
    type Error = AuroriteErrorResponse;
    fn try_from(character: &Character) -> Result<Self, Self::Error> {
        if character.class.is_unloaded() || character.background.is_unloaded() || character.race.is_unloaded() {
            return Err(AuroriteErrorResponse::new("failed to collect data"));
        }
        let background = character.background.get().as_ref().map(BackgroundInfo::from);
        let race = character.race.get().as_ref().map(RaceInfo::from);
        let class = character.class.get().as_ref().map(ClassInfo::from);
        let abilities = CharacterAbilitiesInfo::try_from(character)?;
        Ok(Self {
            id: EncodedUuid(character.id),
            level: character.level,
            max_hits_overwrite: character.max_hits_overwrite,
            background,
            class,
            race,
            abilities,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ClientCharacters {
    pub characters: Vec<CharacterInfo>,
}