use aurorite_dataflow::enums::Skill;
use aurorite_util::uuid::EncodedUuid;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct PostCharacterBase {
    pub level: u8,

    pub name: Option<String>,
    pub full_name: String,

    pub strength: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub charisma: u8,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PutCharacterClass {
    #[schema(value_type = String)]
    pub class_id: EncodedUuid,
    #[schema(value_type = Vec<String>)]
    pub chosen_skills: Vec<Skill>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PutCharacterRace {
    #[schema(value_type = String)]
    pub race_id: EncodedUuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PutCharacterBackground {
    #[schema(value_type = String)]
    pub background_id: EncodedUuid,
}
