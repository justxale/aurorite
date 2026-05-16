use crate::database::{Character, ClassData};
use uuid::Uuid;

#[derive(Clone, Debug, toasty::Model)]
pub struct Class {
    #[key]
    pub id: Uuid,

    pub base_hits: u16,
    pub base_hit_dice: String,

    #[has_many]
    pub characters: toasty::HasMany<Character>,
    #[serialize(json, nullable)]
    pub dyn_data: Option<ClassData>,
}
