use crate::database::{Background, CharacterData, Class, Client, Race, Spell};
use toasty::{Deferred, Json, Model};
use uuid::Uuid;

#[derive(Debug, Clone, Model)]
pub struct CharacterSpell {
    #[key]
    #[index]
    character_id: Uuid,
    #[key]
    #[index]
    spell_id: Uuid,

    pub order: u16,

    #[belongs_to(key = character_id, references = id)]
    pub character: Character,
    #[belongs_to(key = spell_id, references = id)]
    pub spell: Spell,
}

#[derive(Clone, Debug, Model)]
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
    pub client: Deferred<Client>,
    #[belongs_to(key = race_id, references = id)]
    pub race: Deferred<Option<Race>>,
    #[belongs_to(key = class_id, references = id)]
    pub class: Deferred<Option<Class>>,
    #[belongs_to(key = background_id, references = id)]
    pub background: Deferred<Option<Background>>,
    #[has_many]
    pub spells: Deferred<Vec<CharacterSpell>>,

    pub strength: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub charisma: u8,

    pub dyn_data: Option<Json<CharacterData>>,
}
