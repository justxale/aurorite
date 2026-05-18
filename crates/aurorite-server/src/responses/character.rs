use crate::database::ClassData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::database::{Background, BackgroundData, Character, Class, CreatureSize, CreatureType, Race};
use crate::responses::AuroriteErrorResponse;
use crate::utils::get_modification;
use crate::utils::uuid::{encode_uuid};

#[derive(Debug, Serialize)]
pub struct CharacterRaceInfo<'a> {
    id: Uuid,
    l18n: &'a String,
    size: &'a CreatureSize,
    #[serde(rename = "type")]
    creature_type: &'a CreatureType,
    speed: u16,
    dark_vision: Option<u16>
}

impl<'a> CharacterRaceInfo<'a> {
    pub fn new(
        id: Uuid, l18n: &'a String,
        size: &'a CreatureSize, creature_type: &'a CreatureType,
        speed: u16, dark_vision: Option<u16>
    ) -> Self {
        Self {
            id, l18n, size, creature_type, speed, dark_vision
        }
    }
}

impl<'a> From<&'a Race> for CharacterRaceInfo<'a> {
    fn from(race: &'a Race) -> Self {
        Self::new(
            race.id, &race.l18n_key,
            &race.size, &race.creature_type,
            race.speed, race.dark_vision
        )
    }
}

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

#[derive(Debug, Serialize)]
pub struct CharacterInfo {
    id: String
}

#[derive(Debug, Serialize)]
pub struct ClassInfo<'a> {
    id: Uuid,
    l18n_key: &'a String,
    dynamic: Option<&'a ClassData>,

    base_hits: u16,
    base_hit_dice: &'a String,
}

impl<'a> From<&'a Class> for ClassInfo<'a> {
    fn from(class: &'a Class) -> Self {
        Self {
            id: class.id, l18n_key: &class.l18n_key, dynamic: class.dyn_data.as_ref(),
            base_hits: class.base_hits, base_hit_dice: &class.base_hit_dice
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BackgroundInfo<'a> {
    id: Uuid,
    l18n_key: &'a String,
    dynamic: Option<&'a BackgroundData>
}

impl<'a> From<&'a Background> for BackgroundInfo<'a> {
    fn from(background: &'a Background) -> Self {
        Self {
            id: background.id,
            l18n_key: &background.l18n_key,
            dynamic: background.dyn_data.as_ref()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FullCharacterInfo<'a> {
    id: String,
    level: u8,
    max_hits: u16,

    class: ClassInfo<'a>,
    background: BackgroundInfo<'a>,
    race: CharacterRaceInfo<'a>,
    skills: CharacterSkillsInfo,
}

impl<'a> TryFrom<&'a Character> for FullCharacterInfo<'a> {
    type Error = AuroriteErrorResponse;
    fn try_from(char: &'a Character) -> Result<Self, Self::Error> {
        if char.class.is_unloaded() || char.background.is_unloaded() || char.race.is_unloaded() {
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
            ),
            race: char.race.get().into(),
        })
    }
}