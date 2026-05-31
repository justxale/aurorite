use serde::Deserialize;

#[derive(Deserialize)]
pub struct RollQuery {
    pub amount: Option<u16>,
    pub max: Option<u16>,
    pub bonus: Option<i16>,
    pub line: Option<String>,
}