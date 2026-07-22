use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Throw {
    pub source: Uuid,
    pub all: Option<Vec<u16>>,
    pub res: i64,
    pub bonus: i16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitiativeOrder {
    pub target: Uuid,
    pub value: i64,
}

pub enum RuntimeEvent {
    ThrowDice(Throw),
    ThrowDices(Vec<Throw>),
    FinalizeInitiative(Vec<InitiativeOrder>)
}
