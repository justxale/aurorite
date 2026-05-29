use jiff::Timestamp;
use crate::database::{Background, Character, Class, Client, Race, Scene};
use toasty::{BelongsTo, HasMany, Model};
use uuid::Uuid;

#[derive(Clone, Debug, Model)]
pub struct Campaign {
    #[key]
    #[auto]
    pub id: Uuid,
    
    #[default(true)]
    pub is_active: bool,
    pub title: String,

    #[index]
    owner_id: Uuid,
    #[belongs_to(key = owner_id, references = id)]
    owner: BelongsTo<Client>,
    
    #[default(jiff::Timestamp::now())]
    pub last_played_at: Timestamp,
    #[auto]
    pub created_at: Timestamp,

    #[has_many]
    pub clients: HasMany<CampaignClient>,
    #[has_many]
    pub races: HasMany<CampaignRace>,
    #[has_many]
    pub classes: HasMany<CampaignClass>,
    #[has_many]
    pub characters: HasMany<CampaignCharacter>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignCharacter {
    current_hp: u16,
    #[auto]
    #[key]
    pub id: Uuid,
    #[index]
    #[key]
    character_id: Uuid,
    #[index]
    #[key]
    campaign_id: Uuid,

    #[belongs_to(key = character_id, references = id)]
    base: BelongsTo<Character>,
    #[belongs_to(key = campaign_id, references = id)]
    campaign: BelongsTo<Campaign>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignRace {
    #[index]
    #[key]
    race_id: Uuid,
    #[index]
    #[key]
    campaign_id: Uuid,

    #[belongs_to(key = race_id, references = id)]
    race: BelongsTo<Race>,
    #[belongs_to(key = campaign_id, references = id)]
    campaign: BelongsTo<Campaign>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignClass {
    #[index]
    #[key]
    class_id: Uuid,
    #[index]
    #[key]
    campaign_id: Uuid,

    #[belongs_to(key = class_id, references = id)]
    class: BelongsTo<Class>,
    #[belongs_to(key = campaign_id, references = id)]
    campaign: BelongsTo<Campaign>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignBackground {
    #[index]
    #[key]
    background_id: Uuid,
    #[index]
    #[key]
    campaign_id: Uuid,

    #[belongs_to(key = background_id, references = id)]
    class: BelongsTo<Race>,
    #[belongs_to(key = campaign_id, references = id)]
    background: BelongsTo<Background>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignClient {
    #[index]
    #[key]
    client_id: Uuid,
    #[index]
    #[key]
    campaign_id: Uuid,

    #[default(false)]
    pub is_master: bool,

    #[belongs_to(key = client_id, references = id)]
    pub client: BelongsTo<Client>,
    #[belongs_to(key = campaign_id, references = id)]
    pub campaign: BelongsTo<Campaign>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignScene {
    #[key]
    #[auto]
    pub id: Uuid,
    #[index]
    #[key]
    scene_id: Uuid,
    #[index]
    #[key]
    campaign_id: Uuid,

    #[belongs_to(key = scene_id, references = id)]
    pub client: BelongsTo<Scene>,
    #[belongs_to(key = campaign_id, references = id)]
    pub campaign: BelongsTo<Campaign>,

    #[has_many]
    pub preloads: HasMany<PreloadedObjects>
}

#[derive(Clone, Debug, Model)]
pub struct PreloadedObjects {
    #[index]
    #[key]
    scene_id: Uuid,
    #[key]
    character_id: Uuid,

    #[belongs_to(key = scene_id, references = id)]
    pub campaign_scene: BelongsTo<CampaignScene>,
    #[belongs_to(key = character_id, references = id)]
    pub character: BelongsTo<CampaignCharacter>
}
