use crate::database::{Character, ClassData};
use toasty::{Deferred, Json, Model};
use uuid::Uuid;

#[derive(Clone, Debug, Model)]
pub struct Class {
    #[key]
    pub id: Uuid,
    pub l18n_key: String,

    pub base_hits: u16,
    pub base_hit_dice: String,

    #[has_many]
    pub characters: Deferred<Vec<Character>>,
    pub dyn_data: Option<Json<ClassData>>,
}
