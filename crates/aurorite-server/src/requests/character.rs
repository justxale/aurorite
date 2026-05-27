use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PostCharacterBase {
    pub level: u8,

    pub name: Option<String>,
    pub full_name: String,

    pub class: Uuid,
    pub background: Uuid,
    pub race: Uuid,

    pub strength: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub charisma: u8,
}
