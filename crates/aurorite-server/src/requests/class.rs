use aurorite_dataflow::database::ClassData;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct PostClass {
    pub i18n: String,
    pub base_hits: u16,
    pub base_hit_dice: String,
    #[schema(value_type = Option<serde_json::Value>)]
    pub dyn_data: Option<ClassData>,
}
