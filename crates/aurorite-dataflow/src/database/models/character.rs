use crate::database::{Background, CharacterData, Class, Client, Race};
use uuid::Uuid;

#[derive(Clone, Debug, toasty::Model)]
pub struct Character {
    #[key]
    #[auto]
    pub id: Uuid,

    pub name: Option<String>,
    pub full_name: String,

    pub level: u8,
    pub max_hits_overwrite: Option<u16>,

    #[index]
    pub client_id: Uuid,
    #[index]
    class_id: Option<Uuid>,
    #[index]
    background_id: Option<Uuid>,
    #[index]
    race_id: Option<Uuid>,

    #[belongs_to(key = client_id, references = id)]
    pub client: toasty::BelongsTo<Client>,
    #[belongs_to(key = race_id, references = id)]
    pub race: toasty::BelongsTo<Option<Race>>,
    #[belongs_to(key = class_id, references = id)]
    pub class: toasty::BelongsTo<Option<Class>>,
    #[belongs_to(key = background_id, references = id)]
    pub background: toasty::BelongsTo<Option<Background>>,

    pub strength: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub charisma: u8,

    #[serialize(json, nullable)]
    pub dyn_data: Option<CharacterData>,
}
