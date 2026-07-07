use crate::database::{Casting, Character, CharacterSpell, Duration, Materials, Range, School, Script, Spell};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpellDto {
    pub id: Uuid,
    pub i18n: String,
    pub level: u8,
    pub order: u16,
    pub school: School,
    pub materials: Materials,
    pub range: Range,
    pub casting: Casting,
    pub duration: Duration,

    pub script_asset: String,
    pub script_type: Script,

    pub updated_at: Timestamp,
    pub created_at: Timestamp,
}

impl From<CharacterSpell> for SpellDto {
    fn from(record: CharacterSpell) -> Self {
        let spell = record.spell;
        Self {
            id: spell.id,
            i18n: spell.i18n,
            level: spell.level,
            order: record.order,
            school: spell.school,
            materials: spell.materials,
            range: spell.range,
            casting: spell.casting,
            duration: spell.duration,
            script_asset: spell.script.filename,
            script_type: spell.script_type,
            updated_at: spell.updated_at,
            created_at: spell.created_at,
        }
    }
}