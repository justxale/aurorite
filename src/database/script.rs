use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "scripts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub title: String,
    pub content: Option<Json>,
    pub edited_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl ActiveModelBehavior for ActiveModel {}

impl ActiveModel {
    pub fn new(title: &String) -> Self {
        Self {
            id: sea_orm::Set(Uuid::new_v4()),
            title: sea_orm::Set(title.to_owned()),
            content: sea_orm::NotSet,
            created_at: sea_orm::Set(Utc::now()),
            edited_at: sea_orm::Set(Utc::now())
        }
    }
}