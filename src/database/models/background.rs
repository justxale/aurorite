use crate::database::{BackgroundData, Character};
use uuid::Uuid;

#[derive(Clone, Debug, toasty::Model)]
pub struct Background {
    #[key]
    #[auto]
    pub id: Uuid,
    pub l18n_key: String,

    #[has_many]
    pub characters: toasty::HasMany<Character>,
    #[serialize(json, nullable)]
    pub dyn_data: Option<BackgroundData>,
}
