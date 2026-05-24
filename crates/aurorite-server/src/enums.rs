use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Ability {
    Strength,
    Intelligence,
    Wisdom,
    Dexterity,
    Constitution,
    Charisma,
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
    None = 0,
    Base = 1,
    Expert = 2,
}