use aurorite_dataflow::database::ClassData;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostClass {
    pub i18n: String,
    pub base_hits: u16,
    pub base_hit_dice: String,
    pub dyn_data: Option<ClassData>,
}
