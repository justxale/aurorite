use uuid::Uuid;
use serde::{Serialize, Deserialize};
use aurorite_dataflow::dto::{AbilitiesDto, CharacterDto, SkillsDto, SkillDto, AbilityDto};
use aurorite_dataflow::enums::{Ability, Proficiency, Skill};
use aurorite_util::formulas::{get_proficiency_bonus, Dice, DiceRollResult};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Character {
    pub id: Uuid,
    pub controlled_by: Uuid,

    pub name: String,
    pub level: u8,
    pub is_enemy: bool,
    pub is_npc: bool,

    pub max_hits: u16,
    pub current_hits: u16,

    pub mastery: u8,
    pub abilities: AbilitiesDto,
    pub skills: SkillsDto,
}

impl Character {
    pub fn from_dto(dto: CharacterDto, controlled_by: Uuid, is_enemy: bool, is_npc: bool, current_hits: u16) -> Self {
        Self {
            id: dto.id.uuid(),
            controlled_by,

            name: dto.name.unwrap_or(dto.full_name),
            level: dto.level,
            is_enemy, is_npc,
            
            max_hits: dto.max_hits,
            current_hits,
            mastery: get_proficiency_bonus(dto.level),
            skills: dto.skills,
            abilities: dto.abilities,
        }
    }
    
    pub fn get_skill_dto(&self, skill: Skill) -> &SkillDto {
        match skill {
            Skill::Acrobatics => &self.skills.acrobatics,
            Skill::Athletics => &self.skills.athletics,
            Skill::Perception => &self.skills.perception,
            Skill::Survival => &self.skills.survival,
            Skill::Performance => &self.skills.performance,
            Skill::Intimidation => &self.skills.intimidation,
            Skill::History => &self.skills.history,
            Skill::SleightOfHand => &self.skills.sleight_of_hand,
            Skill::Medicine => &self.skills.medicine,
            Skill::Deception => &self.skills.deception,
            Skill::AnimalHandling => &self.skills.animal_handling,
            Skill::Nature => &self.skills.nature,
            Skill::Insight => &self.skills.insight,
            Skill::Investigation => &self.skills.investigation,
            Skill::Religion => &self.skills.religion,
            Skill::Stealth => &self.skills.stealth,
            Skill::Arcana => &self.skills.arcana,
            Skill::Persuasion => &self.skills.persuasion,
        }
    }
    
    pub fn get_ability_dto(&self, ability: Ability) -> &AbilityDto {
        match ability {
            Ability::Strength => &self.abilities.strength,
            Ability::Dexterity => &self.abilities.dexterity,
            Ability::Intelligence => &self.abilities.intelligence,
            Ability::Wisdom => &self.abilities.wisdom,
            Ability::Charisma => &self.abilities.charisma,
            Ability::Constitution => &self.abilities.constitution,
        }
    }

    pub fn skill_dice(&self, skill: Skill) -> Dice {
        let value = self.get_skill_dto(skill);
        let bonus = self.mastery * value.proficiency.as_u8();
        Dice::new(1, 20, Some(value.modification + bonus as i16))
    }

    pub fn ability_dice(&self, ability: Ability) -> Dice {
        let value = self.get_ability_dto(ability);
        Dice::new(1, 20, Some(value.modification))
    }
    
    pub fn save_throw_dice(&self, ability: Ability) -> Dice {
        let value = self.get_ability_dto(ability);
        let bonus = self.mastery * value.save_throw.as_u8();
        Dice::new(1, 20, Some(value.modification + bonus as i16))
    }
}
