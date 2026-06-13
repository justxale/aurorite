use aurorite_dataflow::database::Character;
use aurorite_dataflow::enums::{Ability, Proficiency, Skill};
use crate::responses::character::stats::{AbilityInfo, SkillInfo};
use crate::responses::common::AuroriteErrorResponse;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CharacterAbilitiesInfo {
    strength: AbilityInfo,
    intelligence: AbilityInfo,
    wisdom: AbilityInfo,
    dexterity: AbilityInfo,
    constitution: AbilityInfo,
    charisma: AbilityInfo,
}

impl TryFrom<&Character> for CharacterAbilitiesInfo {
    type Error = AuroriteErrorResponse;
    fn try_from(character: &Character) -> Result<Self, Self::Error> {
        if character.class.is_unloaded() || character.race.is_unloaded() {
            return Err(AuroriteErrorResponse::new("failed to collect data"));
        }

        let mut strength = AbilityInfo::new(character.strength);
        let mut intelligence = AbilityInfo::new(character.intelligence);
        let mut wisdom = AbilityInfo::new(character.wisdom);
        let mut dexterity = AbilityInfo::new(character.dexterity);
        let mut constitution = AbilityInfo::new(character.constitution);
        let mut charisma = AbilityInfo::new(character.charisma);

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
pub struct CharacterSkillsInfo {
    acrobatics: SkillInfo,
    athletics: SkillInfo,
    perception: SkillInfo,
    survival: SkillInfo,
    performance: SkillInfo,
    intimidation: SkillInfo,
    history: SkillInfo,
    sleight_of_hand: SkillInfo,
    medicine: SkillInfo,
    deception: SkillInfo,
    animal_handling: SkillInfo,
    nature: SkillInfo,
    insight: SkillInfo,
    investigation: SkillInfo,
    religion: SkillInfo,
    stealth: SkillInfo,
    arcana: SkillInfo,
    persuasion: SkillInfo,
}

impl TryFrom<&Character> for CharacterSkillsInfo {
    type Error = AuroriteErrorResponse;
    fn try_from(character: &Character) -> Result<Self, Self::Error> {
        if character.class.is_unloaded()
            || character.race.is_unloaded()
            || character.background.is_unloaded()
        {
            return Err(AuroriteErrorResponse::new("failed to collect data"));
        }
        let abilities = CharacterAbilitiesInfo::try_from(character)?;

        let mut athletics = SkillInfo::new(abilities.strength.value);

        let mut acrobatics = SkillInfo::new(abilities.dexterity.value);
        let mut sleight_of_hand = SkillInfo::new(abilities.dexterity.value);
        let mut stealth = SkillInfo::new(abilities.dexterity.value);

        let mut perception = SkillInfo::new(abilities.wisdom.value);
        let mut survival = SkillInfo::new(abilities.wisdom.value);
        let mut medicine = SkillInfo::new(abilities.wisdom.value);
        let mut insight = SkillInfo::new(abilities.wisdom.value);
        let mut animal_handling = SkillInfo::new(abilities.wisdom.value);

        let mut investigation = SkillInfo::new(abilities.intelligence.value);
        let mut history = SkillInfo::new(abilities.intelligence.value);
        let mut arcana = SkillInfo::new(abilities.intelligence.value);
        let mut nature = SkillInfo::new(abilities.intelligence.value);
        let mut religion = SkillInfo::new(abilities.intelligence.value);

        let mut performance = SkillInfo::new(abilities.charisma.value);
        let mut intimidation = SkillInfo::new(abilities.charisma.value);
        let mut deception = SkillInfo::new(abilities.charisma.value);
        let mut persuasion = SkillInfo::new(abilities.charisma.value);

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
