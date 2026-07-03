use crate::database::{Character, RaceData, Spell};
use serde::{Deserialize, Serialize};
use toasty::{Deferred, Embed, Json, Model};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Embed, Serialize, Deserialize)]
pub enum CreatureSize {
    #[column(variant = 1)]
    Tiny,
    #[column(variant = 2)]
    Small,
    #[column(variant = 3)]
    Medium,
    #[column(variant = 4)]
    Large,
    #[column(variant = 5)]
    Huge,
    #[column(variant = 6)]
    Gargantuan,
}

#[derive(Debug, Clone, Copy, Embed, Serialize, Deserialize)]
pub enum CreatureType {
    #[column(variant = 1)]
    Humanoid,
    #[column(variant = 2)]
    Undead,
    #[column(variant = 3)]
    Other,
}

#[derive(Clone, Debug, Model)]
pub struct Race {
    #[key]
    #[auto]
    pub id: Uuid,
    pub i18n: String,

    pub size: CreatureSize,
    #[column("type")]
    pub creature_type: CreatureType,
    pub speed: u16,
    pub dark_vision: Option<u16>,

    #[default(0)]
    pub strength: u8,
    #[default(0)]
    pub intelligence: u8,
    #[default(0)]
    pub wisdom: u8,
    #[default(0)]
    pub dexterity: u8,
    #[default(0)]
    pub constitution: u8,
    #[default(0)]
    pub charisma: u8,

    #[has_many]
    pub spells: Deferred<Vec<RaceSpell>>,
    #[has_many]
    pub characters: Deferred<Vec<Character>>,
    pub dyn_data: Option<Json<RaceData>>,
}

#[derive(Clone, Debug, toasty::Model)]
pub struct RaceSpell {
    #[key]
    #[auto]
    pub id: u64,

    #[index]
    race_id: Uuid,
    #[index]
    spell_id: Uuid,

    #[belongs_to(key = race_id, references = id)]
    race: Deferred<Race>,
    #[belongs_to(key = spell_id, references = id)]
    spell: Deferred<Spell>,
}
