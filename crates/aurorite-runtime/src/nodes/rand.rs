use crate::{try_extract, Character};
use vismut_core::{NodeBuilder, ScriptError, Value, ValueType};
use aurorite_dataflow::enums::{Ability, Skill};
use aurorite_util::formulas::Dice;
use super::{AuroriteCtx, AuroriteNode};
use uuid::Uuid;

fn get_character_field(ctx: &AuroriteCtx, character_id: Uuid, and_then: impl FnOnce(&Character) -> Dice) -> Result<Dice, ScriptError> {
    ctx.lock()
        .map_err(|e| ScriptError::RuntimeError(e.to_string()))?
        .character(character_id)
        .map(and_then)
        .ok_or(ScriptError::RuntimeError(format!("character {} not found", character_id)))
}

fn build_dice_node() -> AuroriteNode {
    NodeBuilder::new("aurorite.rand.general_dice")
        .with_input("dices", &[ValueType::Int])
        .with_input("sides", &[ValueType::Int])
        .with_input("bonus", &[ValueType::Int])
        .with_input("has_advantage", &[ValueType::Bool])
        .with_input("has_disadvantage", &[ValueType::Bool])
        .with_output("result", &[ValueType::BigInt])
        .with_evaluation(|values, _, _| {
            let dices = try_extract!(values, Value::Int, "dices");
            let sides = try_extract!(values, Value::Int, "sides");
            let bonus = try_extract!(values, Value::Int, "bonus");
            let dice = Dice {max: sides as u16, amount: dices as u16, bonus: Some(bonus as i16)};
            Ok(Value::BigInt(dice.roll().sum))
        })
        .build()
}

fn build_save_dice_node() -> AuroriteNode {
    NodeBuilder::new("aurorite.rand.save_dice")
        .with_input("character_id", &[ValueType::Uuid])
        .with_input("ability", &[ValueType::String])
        .with_output("result", &[ValueType::BigInt])
        .with_evaluation(|values, _, ctx: AuroriteCtx| {
            let character_id = try_extract!(values, Value::Uuid, "character_id");
            let ability = try_extract!(values, Value::String, "ability").parse::<Ability>().map_err(|_| ScriptError::UnsupportedInput)?;
            let dice = get_character_field(&ctx, character_id, |character| character.save_throw_dice(ability))?;
            Ok(Value::BigInt(dice.roll().sum))
        })
        .build()
}

fn build_ability_dice_node() -> AuroriteNode {
    NodeBuilder::new("aurorite.rand.ability_dice")
        .with_input("character_id", &[ValueType::Uuid])
        .with_input("ability", &[ValueType::String])
        .with_output("result", &[ValueType::BigInt])
        .with_evaluation(|values, _, ctx: AuroriteCtx| {
            let character_id = try_extract!(values, Value::Uuid, "character_id");
            let ability = try_extract!(values, Value::String, "ability").parse::<Ability>().map_err(|_| ScriptError::UnsupportedInput)?;
            let dice = get_character_field(&ctx, character_id, |character| character.ability_dice(ability))?;
            Ok(Value::BigInt(dice.roll().sum))
        })
        .build()
}

fn build_skill_dice_node() -> AuroriteNode {
    NodeBuilder::new("aurorite.rand.skill_dice")
        .with_input("character_id", &[ValueType::Uuid])
        .with_input("skill", &[ValueType::String])
        .with_output("result", &[ValueType::BigInt])
        .with_evaluation(|values, _, ctx: AuroriteCtx| {
            let character_id = try_extract!(values, Value::Uuid, "character_id");
            let skill = try_extract!(values, Value::String, "skill").parse::<Skill>().map_err(|_| ScriptError::UnsupportedInput)?;
            let dice = get_character_field(&ctx, character_id, |character| character.skill_dice(skill))?;
            Ok(Value::BigInt(dice.roll().sum))
        })
        .build()
}

pub fn build_rand_nodes() -> Vec<AuroriteNode> {
    vec![build_dice_node(), build_skill_dice_node(), build_ability_dice_node(), build_save_dice_node()]
}