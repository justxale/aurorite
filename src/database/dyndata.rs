use crate::enums::{Ability, ArmourType, InstrumentType, Skill, WeaponType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundData {
    skills: Vec<Skill>,
    instrument_type: InstrumentType,
    // feat
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassData {
    abilities: Vec<Ability>,
    save_throws: Vec<Ability>,
    hit_dice: String,
    hit_default: u8,
    hits_per_level: String,

    skills: Vec<Skill>,
    skills_amount: u8,
    weapon_types: Vec<WeaponType>,
    armour_types: Vec<ArmourType>,
    instrument_types: Vec<InstrumentType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceData {

}