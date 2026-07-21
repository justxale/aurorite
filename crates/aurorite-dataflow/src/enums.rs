use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Skill {
    Acrobatics,
    Athletics,
    Perception,
    Survival,
    Performance,
    Intimidation,
    History,
    SleightOfHand,
    Medicine,
    Deception,
    AnimalHandling,
    Nature,
    Insight,
    Investigation,
    Religion,
    Stealth,
    Arcana,
    Persuasion,
}

impl FromStr for Skill {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "acrobatics" => Skill::Acrobatics,
            "athletics" => Skill::Athletics,
            "perception" => Skill::Perception,
            "survival" => Skill::Survival,
            "performance" => Skill::Performance,
            "intimidation" => Skill::Intimidation,
            "history" => Skill::History,
            "sleight_of_hand" => Skill::SleightOfHand,
            "medicine" => Skill::Medicine,
            "deception" => Skill::Deception,
            "animal_handling" => Skill::AnimalHandling,
            "nature" => Skill::Nature,
            "investigation" => Skill::Investigation,
            "religion" => Skill::Religion,
            "stealth" => Skill::Stealth,
            "arcana" => Skill::Arcana,
            "persuasion" => Skill::Persuasion,
            &_ => return Err(()),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Ability {
    Strength,
    Intelligence,
    Wisdom,
    Dexterity,
    Constitution,
    Charisma,
}

impl FromStr for Ability {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "strength" => Ability::Strength,
            "intelligence" => Ability::Intelligence,
            "wisdom" => Ability::Wisdom,
            "dexterity" => Ability::Dexterity,
            "constitution" => Ability::Constitution,
            "charisma" => Ability::Charisma,
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum WeaponType {
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum ArmourType {
    LightArmour,
    MediumArmour,
    HeavyArmour,
    Shield,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum InstrumentType {
    Artisan,
    GamingSet,
    Musical,
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Proficiency {
    None,
    Base,
    Expert,
}

impl Proficiency {
    pub fn as_u8(&self) -> u8 {
        match self {
            Proficiency::None => 0,
            Proficiency::Base => 1,
            Proficiency::Expert => 2,
        }
    }
}
