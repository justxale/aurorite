use crate::database::{Background, Class, Client};
use uuid::Uuid;

#[derive(Clone, Debug, toasty::Model)]
pub struct Character {
    #[key]
    #[auto]
    pub id: Uuid,

    pub level: u8,
    pub max_hits: u16,
    pub current_hits: u16,

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

    strength: u8,
    intelligence: u8,
    wisdom: u8,
    dexterity: u8,
    constitution: u8,
    charisma: u8,
}
