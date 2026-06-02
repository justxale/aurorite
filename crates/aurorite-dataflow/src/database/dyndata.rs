use crate::enums::{Ability, ArmourType, InstrumentType, Proficiency, Skill, WeaponType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundData {
    pub skills: Vec<Skill>,
    pub instrument_type: InstrumentType,
    // feat
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassData {
    // abilities: Vec<Ability>,
    pub save_throws: Vec<Ability>,
    pub hit_dice: String,
    pub hit_default: u8,
    pub hits_per_level: String,

    pub skills: Vec<Skill>,
    pub skills_amount: u8,
    pub weapon_types: Vec<WeaponType>,
    pub armour_types: Vec<ArmourType>,
    pub instrument_types: Vec<InstrumentType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceData {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Overwrite<K, V> {
    pub kind: K,
    pub value: Option<V>,
    pub proficiency: Option<Proficiency>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterData {
    pub chosen_class_skills: Vec<Skill>,
    pub skill_overwrites: Vec<Overwrite<Skill, u8>>,
    pub ability_overwrites: Vec<Overwrite<Ability, u8>>,
}
