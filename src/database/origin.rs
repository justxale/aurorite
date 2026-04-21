use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel)]
#[sea_orm(table_name = "origin")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub l18n_key: String,

    #[sea_orm(has_many)]
    pub class: HasMany<super::character::Entity>,

    pub strength: i8,
    pub intelligence: i8,
    pub wisdom: i8,
    pub dexterity: i8,
    pub constitution: i8,
    pub charisma: i8
}

impl ActiveModelBehavior for ActiveModel {}