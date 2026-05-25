use toasty::{BelongsTo, HasMany, Model};
use uuid::Uuid;
use crate::database::{Background, Character, Class, Client, Race};

#[derive(Clone, Debug, Model)]
pub struct Campaign {
    #[key]
    #[auto]
    pub id: Uuid,
    #[index]
    owner_id: Uuid,

    pub title: String,

    #[has_many]
    pub clients: HasMany<CampaignClient>,
    #[has_many]
    pub races: HasMany<CampaignRace>,
    #[has_many]
    pub classes: HasMany<CampaignClass>,

    #[belongs_to(key = owner_id, references = id)]
    owner: BelongsTo<Client>
}

#[derive(Clone, Debug, Model)]
pub struct CampaignCharacter {
    #[key]
    #[auto]
    pub id: Uuid,

    current_hp: u16,

    #[index]
    character_id: Uuid,
    #[belongs_to(key = character_id, references = id)]
    base: BelongsTo<Character>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignRace {
    #[key]
    #[auto]
    pub id: u64,

    #[index]
    race_id: Uuid,
    #[index]
    campaign_id: Uuid,

    #[belongs_to(key = race_id, references = id)]
    race: BelongsTo<Race>,
    #[belongs_to(key = campaign_id, references = id)]
    campaign: BelongsTo<Campaign>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignClass {
    #[key]
    #[auto]
    pub id: u64,

    #[index]
    class_id: Uuid,
    #[index]
    campaign_id: Uuid,

    #[belongs_to(key = class_id, references = id)]
    class: BelongsTo<Class>,
    #[belongs_to(key = campaign_id, references = id)]
    campaign: BelongsTo<Campaign>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignBackground {
    #[key]
    #[auto]
    pub id: u64,

    #[index]
    background_id: Uuid,
    #[index]
    campaign_id: Uuid,

    #[belongs_to(key = background_id, references = id)]
    class: BelongsTo<Race>,
    #[belongs_to(key = campaign_id, references = id)]
    background: BelongsTo<Background>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignClient {
    #[key]
    client_id: Uuid,
    #[key]
    campaign_id: Uuid,

    #[default(false)]
    pub is_master: bool,

    #[belongs_to(key = client_id, references = id)]
    pub client: BelongsTo<Client>,
    #[belongs_to(key = campaign_id, references = id)]
    pub campaign: BelongsTo<Campaign>,
}