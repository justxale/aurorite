use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "class")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub base_hits: u16,

    #[sea_orm(has_many)]
    pub characters: HasMany<super::character::Entity>,
}

impl ActiveModelBehavior for ActiveModel {}