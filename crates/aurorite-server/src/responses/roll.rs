use aurorite_util::formulas::DiceRollResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RollResult {
    pub parsed_dice: u16,
    pub parsed_amount: u16,
    pub parsed_bonus: Option<i16>,
    pub result: DiceRollResult,
}
