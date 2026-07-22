use uuid::Uuid;

pub struct Throw {
    pub source: Uuid,
    pub all: Option<Vec<u16>>,
    pub res: i64,
    pub bonus: i16
}

pub struct InitiativeOrder {
    pub target: Uuid,
    pub value: i64,
}

pub enum RuntimeEvent {
    ThrowDice(Throw),
    ThrowDices(Vec<Throw>),
    FinalizeInitiative(Vec<InitiativeOrder>)
}
