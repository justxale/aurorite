use crate::database::{BackgroundData, Character};
use toasty::{Model, Deferred, Json};
use uuid::Uuid;

#[derive(Clone, Debug, Model)]
pub struct Background {
    #[key]
    #[auto]
    pub id: Uuid,
    pub l18n_key: String,

    #[has_many]
    pub characters: Deferred<Vec<Character>>,
    pub dyn_data: Option<Json<BackgroundData>>,
}
