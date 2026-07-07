use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub fn get_modification(value: u8) -> i16 {
    if value <= 11 {
        (value as i16 - 11) / 2
    } else {
        (value as i16 - 10) / 2
    }
}

pub fn get_proficiency_bonus(level: u8) -> u8 {
    2 + ((level - 1) / 4)
}

pub struct Dice {
    pub amount: u16,
    pub max: u16,
    pub bonus: Option<i16>,
}

#[derive(Serialize, Deserialize)]
pub struct DiceRollResult {
    pub all: Vec<u16>,
    pub sum: i64,
}

impl Dice {
    pub fn new(amount: u16, max: u16, bonus: Option<i16>) -> Self {
        Self { amount, max, bonus }
    }

    pub fn roll(&self) -> DiceRollResult {
        let all = fastrand::choose_multiple(1..=self.max, self.amount as usize);
        let sum = all
            .iter()
            .fold(0i64, |tmp, v| tmp + *v as i64) + self.bonus.unwrap_or(0) as i64;
        DiceRollResult {
            sum,
            all,
        }
    }
}

impl FromStr for Dice {
    type Err = String;
    fn from_str(query: &str) -> Result<Self, Self::Err> {
        let (amount, rest) = match query.split_once('d') {
            None => return Err("invalid roll query".to_string()),
            Some((amount, rest)) => (amount.parse::<u16>().map_err(|_| format!("invalid amount: {amount}",))?, rest),
        };
        let (max, bonus) = match rest.find(['+', '-']) {
            Some(pos) => {
                let sides = rest[..pos].parse::<u16>().map_err(|_| format!("invalid max: {}", &rest[..pos]))?;
                let bonus = Some(rest[pos..].parse::<i16>().map_err(|_| format!("invalid bonus: {}", &rest[pos..]))?);
                (sides, bonus)
            }
            None => (rest.parse::<u16>().map_err(|_| format!("invalid max: {rest}"))?, None),
        };
        Ok(Self::new(amount, max, bonus))
    }
}
