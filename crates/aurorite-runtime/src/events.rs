use serde::{Deserialize, Serialize};
use aurorite_util::uuid::EncodedUuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThrowEntry {
    pub source: EncodedUuid,
    pub all: Option<Vec<u16>>,
    pub res: i64,
    pub bonus: i16
}

pub enum RuntimeEvent {
    ThrowDice(ThrowEntry),
    ThrowDices(Vec<ThrowEntry>),
    
    LoadInitiative,
    UnloadInitiative,
    
    LoadScene(EncodedUuid),
    UnloadScene,
}
