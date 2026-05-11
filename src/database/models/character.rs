use crate::database::{Background, Class, Client};
use uuid::Uuid;

#[derive(Clone, Debug, toasty::Model)]
pub struct Character {
    #[key]
    #[auto]
    pub id: Uuid,

    pub level: u8,
    pub max_hits: u16,
    // pub current_hits: u16,

    #[index]
    class_id: Uuid,
    #[index]
    background_id: Uuid,
    #[index]
    client_id: Uuid,

    #[belongs_to(key = class_id, references = id)]
    pub class: toasty::BelongsTo<Class>,
    #[belongs_to(key = background_id, references = id)]
    pub background: toasty::BelongsTo<Background>,
    #[belongs_to(key = client_id, references = id)]
    pub client: toasty::BelongsTo<Client>,

    pub strength: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub charisma: u8,
}
