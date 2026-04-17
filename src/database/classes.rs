use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "class")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(belongs_to, from = "id", to = "id")]
    pub characters: HasOne<super::character::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}