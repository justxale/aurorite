use jiff::Timestamp;
use uuid::Uuid;

#[derive(Clone, Debug, toasty::Model)]
pub struct Spell {
    #[key]
    #[auto]
    pub id: Uuid,

    pub title: String,
    pub description: Option<String>,

    #[auto]
    pub updated_at: Timestamp,
    #[auto]
    pub created_at: Timestamp,
}
