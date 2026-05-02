use crate::database::Character;
use jiff::Timestamp;
use uuid::Uuid;

#[derive(Clone, Debug, toasty::Model)]
pub struct Client {
    #[key]
    #[auto]
    pub id: Uuid,
    #[unique]
    pub nickname: String,
    pub pwd: String,

    pub display_name: String,

    #[auto]
    pub updated_at: Timestamp,
    #[auto]
    pub created_at: Timestamp,

    #[has_many]
    pub characters: toasty::HasMany<Character>,
}
