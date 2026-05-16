use serde::{Deserialize, Serialize};
use crate::database::{Background, Character, Class};
use crate::responses::AuroriteErrorResponse;
use crate::utils::get_modification;
use crate::utils::uuid::{encode_uuid};

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillInfo {
    value: u8,
    modification: i16,
}

impl SkillInfo {
    pub fn new(value: u8) -> Self {
        Self {
            value, modification: get_modification(value.into())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterSkillsInfo {
    strength: SkillInfo,
    intelligence: SkillInfo,
    wisdom: SkillInfo,
    dexterity: SkillInfo,
    constitution: SkillInfo,
    charisma: SkillInfo,
}

impl CharacterSkillsInfo {
    pub fn new(str: u8, int: u8, wsd: u8, dex: u8, con: u8, chr: u8) -> Self {
        Self {
            strength: SkillInfo::new(str),
            intelligence: SkillInfo::new(int),
            wisdom: SkillInfo::new(wsd),
            dexterity: SkillInfo::new(dex),
            constitution: SkillInfo::new(con),
            charisma: SkillInfo::new(chr)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacterInfo {
    id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassInfo {

}

impl From<&Class> for ClassInfo {
    fn from(class: &Class) -> Self {
        Self {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackgroundInfo {

}

impl From<&Background> for BackgroundInfo {
    fn from(background: &Background) -> Self {
        Self {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullCharacterInfo {
    id: String,
    level: u8,
    max_hits: u16,

    background: BackgroundInfo,
    class: ClassInfo,
    skills: CharacterSkillsInfo,
}

impl TryFrom<Character> for FullCharacterInfo {
    type Error = AuroriteErrorResponse;
    fn try_from(char: Character) -> Result<Self, Self::Error> {
        if char.class.is_unloaded() || char.background.is_unloaded() {
            return Err(AuroriteErrorResponse::new("failed to collect data"))
        }
        Ok(FullCharacterInfo {
            id: encode_uuid(&char.id),
            level: char.level,
            max_hits: char.max_hits,
            background: char.background.get().into(),
            class: char.class.get().into(),
            skills: CharacterSkillsInfo::new(
                char.strength,
                char.intelligence,
                char.wisdom,
                char.dexterity,
                char.constitution,
                char.charisma
            )
        })
    }
}