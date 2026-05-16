use jiff::Timestamp;
use uuid::Uuid;

#[derive(Clone, Debug, toasty::Model)]
pub struct Script {
    #[key]
    #[auto]
    pub id: Uuid,
    #[unique]
    pub title: String,
    // pub content: Option<Json>,
    #[auto]
    pub updated_at: Timestamp,
    #[auto]
    pub created_at: Timestamp,
}
