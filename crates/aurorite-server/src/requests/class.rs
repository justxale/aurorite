use crate::database::ClassData;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostClass {
    pub l18n: String,
    pub base_hits: u16,
    pub base_hit_dice: String,
    pub dyn_data: Option<ClassData>,
}
