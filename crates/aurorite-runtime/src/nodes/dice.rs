use std::sync::{Arc, Mutex};
use vismut_core::{BuiltNode, NodeBuilder, Value, ValueType};
use aurorite_util::formulas::Dice;
use crate::RuntimeCtx;
use super::try_extract;

fn build_dice_node() -> BuiltNode<Arc<Mutex<RuntimeCtx>>> {
    NodeBuilder::new("dice")
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