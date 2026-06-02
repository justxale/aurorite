use aurorite_dataflow::database::Overwrite;
use aurorite_dataflow::enums::{Ability, Proficiency, Skill};
use aurorite_util::formulas::get_modification;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct AbilityInfo {
    pub value: u8,
    modification: i8,
    pub save_throw: Proficiency,
}

impl AbilityInfo {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            modification: get_modification(value),
            save_throw: Proficiency::None,
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
        self.modification = get_modification(value);
    }

    pub fn set_overwrite(&mut self, overwrite: &Overwrite<Ability, u8>) {
        if let Some(v) = overwrite.value {
            self.set_value(v);
        }
        if let Some(p) = overwrite.proficiency {
            self.save_throw = p;
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct SkillInfo {
    value: u8,
    modification: i8,
    pub proficiency: Proficiency,
}

impl SkillInfo {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            modification: get_modification(value),
            proficiency: Proficiency::None,
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
        self.modification = get_modification(value);
    }

    pub fn set_overwrite(&mut self, overwrite: &Overwrite<Skill, u8>) {
        if let Some(v) = overwrite.value {
            self.set_value(v);
        }
        if let Some(p) = overwrite.proficiency {
            self.proficiency = p;
        }
    }
}
