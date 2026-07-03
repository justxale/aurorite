use crate::database::{Asset, Background, Character, Class, Client, Race};
use aurorite_util::common::create_hex;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use toasty::{Deferred, Embed, Model};
use uuid::Uuid;

#[derive(Clone, Debug, Embed, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    #[column(variant = 0)]
    Private,
    #[column(variant = 1)]
    InviteOnly,
}

#[derive(Clone, Debug, Embed, Deserialize, Serialize)]
pub struct AccessState {
    pub visibility: Visibility,
    pub code: Option<String>,
}

impl AccessState {
    pub fn invite_only() -> Self {
        Self {
            visibility: Visibility::InviteOnly,
            code: Some(create_hex::<12>()),
        }
    }

    pub fn private() -> Self {
        Self {
            visibility: Visibility::Private,
            code: None,
        }
    }
}

impl Default for AccessState {
    fn default() -> Self {
        Self::private()
    }
}

#[derive(Clone, Debug, Model)]
pub struct Campaign {
    #[key]
    #[auto]
    pub id: Uuid,

    pub title: String,
    #[default(true)]
    pub is_active: bool,
    #[default(AccessState::default())]
    pub access_state: AccessState,

    #[index]
    owner_id: Uuid,
    #[index]
    scene_id: Option<Uuid>,

    #[belongs_to(key = owner_id, references = id)]
    pub owner: Deferred<Client>,
    #[belongs_to(key = scene_id, references = id)]
    pub scene: Deferred<Option<Scene>>,

    #[default(jiff::Timestamp::now())]
    pub last_played_at: Timestamp,
    #[auto]
    pub created_at: Timestamp,

    #[has_many]
    pub clients: Deferred<Vec<CampaignClient>>,
    #[has_many]
    pub races: Deferred<Vec<CampaignRace>>,
    #[has_many]
    pub classes: Deferred<Vec<CampaignClass>>,
    #[has_many]
    pub characters: Deferred<Vec<CampaignCharacter>>,
}

#[derive(Clone, Debug, Model)]
pub struct CampaignCharacter {
    current_hits: u16,
    #[index]
    #[key]
    character_id: Uuid,
    #[index]
    #[key]
    campaign_id: Uuid,

    #[belongs_to(key = character_id, references = id)]
    base: Deferred<Character>,
    #[belongs_to(key = campaign_id, references = id)]
    campaign: Deferred<Campaign>,
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
    race: Deferred<Race>,
    #[belongs_to(key = campaign_id, references = id)]
    campaign: Deferred<Campaign>,
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
    class: Deferred<Class>,
    #[belongs_to(key = campaign_id, references = id)]
    campaign: Deferred<Campaign>,
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
    class: Deferred<Race>,
    #[belongs_to(key = campaign_id, references = id)]
    background: Deferred<Background>,
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
    pub client: Deferred<Client>,
    #[belongs_to(key = campaign_id, references = id)]
    pub campaign: Deferred<Campaign>,
}

#[derive(Clone, Debug, Model)]
pub struct Scene {
    #[key]
    #[auto]
    pub id: Uuid,

    #[index]
    asset_id: Option<Uuid>,
    #[index]
    campaign_id: Uuid,

    #[belongs_to(key = campaign_id, references = id)]
    pub campaign: Deferred<Campaign>,
    #[belongs_to(key = asset_id, references = id)]
    pub asset: Deferred<Option<Asset>>,

    #[has_many]
    pub preloads: Deferred<Vec<PreloadedObject>>,
}

#[derive(Clone, Debug, Model)]
pub struct PreloadedObject {
    #[index]
    #[key]
    scene_id: Uuid,
    #[index]
    #[key]
    pub character_id: Uuid,
    #[index]
    #[key]
    pub campaign_id: Uuid,

    #[default(true)]
    pub is_visible: bool,

    #[belongs_to(key = scene_id, references = id)]
    pub scene: Deferred<Scene>,
    #[belongs_to(key = [character_id, campaign_id], references = [character_id, campaign_id])]
    pub character: Deferred<CampaignCharacter>,
}
