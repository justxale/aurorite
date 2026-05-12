use uuid::Uuid;
use crate::database::{Character, RaceData};

#[derive(Debug, Clone)]
pub enum CreatureSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan
}

#[derive(Debug, Clone)]
pub enum CreatureType {
    Humanoid,
    Undead,
    Other
}

#[derive(Clone, Debug, toasty::Model)]
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
    pub characters: toasty::HasMany<Character>,
    #[serialize(json, nullable)]
    pub dyn_data: Option<RaceData>,
}