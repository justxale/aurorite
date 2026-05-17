use uuid::Uuid;
use toasty::{Embed, Model};
use serde::{Serialize, Deserialize};
use crate::database::{Character, RaceData, Spell};

#[derive(Debug, Clone, Embed, Serialize, Deserialize)]
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
    Gargantuan
}

#[derive(Debug, Clone, Embed, Serialize, Deserialize)]
pub enum CreatureType {
    #[column(variant = 1)]
    Humanoid,
    #[column(variant = 2)]
    Undead,
    #[column(variant = 3)]
    Other
}

#[derive(Clone, Debug, Model)]
pub struct Race {
    #[key]
    #[auto]
    pub id: Uuid,
    pub l18n_key: String,

    pub size: CreatureSize,
    #[column("type")]
    pub creature_type: CreatureType,
    pub speed: u16,
    pub dark_vision: Option<u16>,

    #[has_many]
    pub spells: toasty::HasMany<RaceSpell>,
    #[has_many]
    pub characters: toasty::HasMany<Character>,
    #[serialize(json, nullable)]
    pub dyn_data: Option<RaceData>,
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
    race: toasty::BelongsTo<Race>,
    #[belongs_to(key = spell_id, references = id)]
    spell: toasty::BelongsTo<Spell>
}