use toasty::Model;
use uuid::Uuid;

#[derive(Debug, Clone, Model)]
pub struct Asset {
    #[key]
    pub id: Uuid,
    #[unique]
    pub filename: String,
    #[column(type = varchar(64))]
    pub hash: String
}