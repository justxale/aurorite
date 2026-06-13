use crate::database::{Character, Overwrite};
use crate::enums::{Ability, Proficiency, Skill};
use aurorite_util::formulas::get_modification;
use serde::{Deserialize, Serialize};
use aurorite_util::uuid::EncodedUuid;
use crate::dto::background::BackgroundDto;
use crate::dto::class::ClassObj;
use crate::dto::race::RaceDto;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct AbilityDto {
    pub value: u8,
    modification: i8,
    pub save_throw: Proficiency,
}

impl AbilityDto {
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
pub struct SkillDto {
    value: u8,
    modification: i8,
    pub proficiency: Proficiency,
}

impl SkillDto {
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


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct AbilitiesDto {
    strength: AbilityDto,
    intelligence: AbilityDto,
    wisdom: AbilityDto,
    dexterity: AbilityDto,
    constitution: AbilityDto,
    charisma: AbilityDto,
}

impl TryFrom<&Character> for AbilitiesDto {
    type Error = &'static str;
    fn try_from(character: &Character) -> Result<Self, Self::Error> {
        if character.class.is_unloaded() || character.race.is_unloaded() {
            return Err("failed to collect data");
        }

        let mut strength = AbilityDto::new(character.strength);
        let mut intelligence = AbilityDto::new(character.intelligence);
        let mut wisdom = AbilityDto::new(character.wisdom);
        let mut dexterity = AbilityDto::new(character.dexterity);
        let mut constitution = AbilityDto::new(character.constitution);
        let mut charisma = AbilityDto::new(character.charisma);

        if let Some(class_data) = character.class.get()
            && let Some(ref data) = class_data.dyn_data
        {
            for save_throw in &data.save_throws {
                match save_throw {
                    Ability::Strength => strength.save_throw = Proficiency::Base,
                    Ability::Intelligence => intelligence.save_throw = Proficiency::Base,
                    Ability::Wisdom => wisdom.save_throw = Proficiency::Base,
                    Ability::Dexterity => dexterity.save_throw = Proficiency::Base,
                    Ability::Constitution => constitution.save_throw = Proficiency::Base,
                    Ability::Charisma => charisma.save_throw = Proficiency::Base,
                }
            }
        }

        if let Some(race) = character.race.get() {
            strength.set_value(character.strength + race.strength);
            intelligence.set_value(character.intelligence + race.intelligence);
            wisdom.set_value(character.wisdom + race.wisdom);
            dexterity.set_value(character.dexterity + race.dexterity);
            constitution.set_value(character.constitution + race.constitution);
            charisma.set_value(character.charisma + race.charisma);
        }

        if let Some(data) = &character.dyn_data {
            for overwrite in &data.ability_overwrites {
                match overwrite.kind {
                    Ability::Strength => strength.set_overwrite(overwrite),
                    Ability::Intelligence => intelligence.set_overwrite(overwrite),
                    Ability::Wisdom => wisdom.set_overwrite(overwrite),
                    Ability::Dexterity => dexterity.set_overwrite(overwrite),
                    Ability::Constitution => constitution.set_overwrite(overwrite),
                    Ability::Charisma => charisma.set_overwrite(overwrite),
                }
            }
        }

        Ok(Self {
            strength,
            intelligence,
            wisdom,
            dexterity,
            constitution,
            charisma,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillsDto {
    acrobatics: SkillDto,
    athletics: SkillDto,
    perception: SkillDto,
    survival: SkillDto,
    performance: SkillDto,
    intimidation: SkillDto,
    history: SkillDto,
    sleight_of_hand: SkillDto,
    medicine: SkillDto,
    deception: SkillDto,
    animal_handling: SkillDto,
    nature: SkillDto,
    insight: SkillDto,
    investigation: SkillDto,
    religion: SkillDto,
    stealth: SkillDto,
    arcana: SkillDto,
    persuasion: SkillDto,
}

impl TryFrom<&Character> for SkillsDto {
    type Error = &'static str;
    fn try_from(character: &Character) -> Result<Self, Self::Error> {
        if character.class.is_unloaded()
            || character.race.is_unloaded()
            || character.background.is_unloaded()
        {
            return Err("failed to collect data");
        }
        let abilities = AbilitiesDto::try_from(character)?;

        let mut athletics = SkillDto::new(abilities.strength.value);

        let mut acrobatics = SkillDto::new(abilities.dexterity.value);
        let mut sleight_of_hand = SkillDto::new(abilities.dexterity.value);
        let mut stealth = SkillDto::new(abilities.dexterity.value);

        let mut perception = SkillDto::new(abilities.wisdom.value);
        let mut survival = SkillDto::new(abilities.wisdom.value);
        let mut medicine = SkillDto::new(abilities.wisdom.value);
        let mut insight = SkillDto::new(abilities.wisdom.value);
        let mut animal_handling = SkillDto::new(abilities.wisdom.value);

        let mut investigation = SkillDto::new(abilities.intelligence.value);
        let mut history = SkillDto::new(abilities.intelligence.value);
        let mut arcana = SkillDto::new(abilities.intelligence.value);
        let mut nature = SkillDto::new(abilities.intelligence.value);
        let mut religion = SkillDto::new(abilities.intelligence.value);

        let mut performance = SkillDto::new(abilities.charisma.value);
        let mut intimidation = SkillDto::new(abilities.charisma.value);
        let mut deception = SkillDto::new(abilities.charisma.value);
        let mut persuasion = SkillDto::new(abilities.charisma.value);

        let mut selected_skills: Vec<Skill> = Vec::new();

        if let Some(data) = &character.dyn_data {
            selected_skills.extend_from_slice(&data.chosen_class_skills)
        }
        if let Some(class_data) = &character.background.get()
            && let Some(ref data) = class_data.dyn_data
        {
            selected_skills.extend_from_slice(&data.skills)
        }

        for skill in &selected_skills {
            match skill {
                Skill::Acrobatics => acrobatics.proficiency = Proficiency::Base,
                Skill::Athletics => athletics.proficiency = Proficiency::Base,
                Skill::Perception => perception.proficiency = Proficiency::Base,
                Skill::Survival => survival.proficiency = Proficiency::Base,
                Skill::Performance => performance.proficiency = Proficiency::Base,
                Skill::Intimidation => intimidation.proficiency = Proficiency::Base,
                Skill::History => history.proficiency = Proficiency::Base,
                Skill::SleightOfHand => sleight_of_hand.proficiency = Proficiency::Base,
                Skill::Medicine => medicine.proficiency = Proficiency::Base,
                Skill::Deception => deception.proficiency = Proficiency::Base,
                Skill::AnimalHandling => animal_handling.proficiency = Proficiency::Base,
                Skill::Nature => nature.proficiency = Proficiency::Base,
                Skill::Insight => insight.proficiency = Proficiency::Base,
                Skill::Investigation => investigation.proficiency = Proficiency::Base,
                Skill::Religion => religion.proficiency = Proficiency::Base,
                Skill::Stealth => stealth.proficiency = Proficiency::Base,
                Skill::Arcana => arcana.proficiency = Proficiency::Base,
                Skill::Persuasion => persuasion.proficiency = Proficiency::Base,
            }
        }
        if let Some(data) = &character.dyn_data {
            for overwrite in &data.skill_overwrites {
                match overwrite.kind {
                    Skill::Acrobatics => acrobatics.set_overwrite(overwrite),
                    Skill::Athletics => athletics.set_overwrite(overwrite),
                    Skill::Perception => perception.set_overwrite(overwrite),
                    Skill::Survival => survival.set_overwrite(overwrite),
                    Skill::Performance => performance.set_overwrite(overwrite),
                    Skill::Intimidation => intimidation.set_overwrite(overwrite),
                    Skill::History => history.set_overwrite(overwrite),
                    Skill::SleightOfHand => sleight_of_hand.set_overwrite(overwrite),
                    Skill::Medicine => medicine.set_overwrite(overwrite),
                    Skill::Deception => deception.set_overwrite(overwrite),
                    Skill::AnimalHandling => animal_handling.set_overwrite(overwrite),
                    Skill::Nature => nature.set_overwrite(overwrite),
                    Skill::Insight => insight.set_overwrite(overwrite),
                    Skill::Investigation => investigation.set_overwrite(overwrite),
                    Skill::Religion => religion.set_overwrite(overwrite),
                    Skill::Stealth => stealth.set_overwrite(overwrite),
                    Skill::Arcana => arcana.set_overwrite(overwrite),
                    Skill::Persuasion => persuasion.set_overwrite(overwrite),
                }
            }
        }

        Ok(Self {
            acrobatics,
            athletics,
            perception,
            survival,
            performance,
            intimidation,
            history,
            sleight_of_hand,
            medicine,
            deception,
            animal_handling,
            nature,
            insight,
            investigation,
            religion,
            stealth,
            arcana,
            persuasion,
        })
    }
}

#[derive(Serialize)]
pub struct CharacterDto {
    id: EncodedUuid,
    name: Option<String>,
    full_name: String,
    level: u8,
    max_hits_overwrite: Option<u16>,

    class: Option<ClassObj>,
    background: Option<BackgroundDto>,
    race: Option<RaceDto>,
    abilities: AbilitiesDto,
    skills: SkillsDto,
}

impl TryFrom<&Character> for CharacterDto {
    type Error = &'static str;
    fn try_from(character: &Character) -> Result<Self, Self::Error> {
        if character.class.is_unloaded()
            || character.background.is_unloaded()
            || character.race.is_unloaded()
        {
            return Err("failed to collect data");
        }
        let background = character
            .background
            .get()
            .as_ref()
            .map(BackgroundDto::from);
        let race = character.race.get().as_ref().map(RaceDto::from);
        let class = character.class.get().as_ref().map(ClassObj::from);
        let abilities = AbilitiesDto::try_from(character)?;
        let skills = SkillsDto::try_from(character)?;
        Ok(Self {
            full_name: character.full_name.clone(),
            name: character.name.clone(),
            id: EncodedUuid(character.id),
            level: character.level,
            max_hits_overwrite: character.max_hits_overwrite,
            background,
            class,
            race,
            abilities,
            skills,
        })
    }
}