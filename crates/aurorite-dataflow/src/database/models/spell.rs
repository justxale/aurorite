use crate::database::Asset;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, toasty::Embed, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Script {
    #[column(variant = 0)]
    Vismut,
}

#[derive(Debug, Clone, toasty::Embed, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RangeType {
    #[column(variant = "self")]
    #[serde(rename = "self")]
    Caster,
    #[column(variant = "touch")]
    Touch,
    #[column(variant = "ranged")]
    Range,
    Cone,
}

#[derive(Debug, Clone, toasty::Embed, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Range {
    #[column("type")]
    #[serde(rename = "type")]
    range_type: RangeType,
    value: Option<u16>,
}

#[derive(Debug, Clone, toasty::Embed, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeUnit {
    #[column(variant = "rounds")]
    Rounds,
    #[column(variant = "actions")]
    Actions,
    #[column(variant = "minutes")]
    Minutes,
    #[column(variant = "hours")]
    Hours,
}

#[derive(Debug, Clone, toasty::Embed, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum School {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

#[derive(Debug, Clone, toasty::Embed, Deserialize, Serialize)]
pub struct Casting {
    pub unit: TimeUnit,
    pub time: u16,
}

#[derive(Debug, Clone, toasty::Embed, Deserialize, Serialize)]
pub struct Duration {
    pub unit: TimeUnit,
    pub time: u16,
    pub has_concentration: bool,
}

#[derive(Debug, Clone, toasty::Embed, Deserialize, Serialize)]
pub struct Materials {
    pub verbal: bool,
    pub somatic: bool,
    pub material: bool,
}

#[derive(Clone, Debug, toasty::Model)]
pub struct Spell {
    #[key]
    #[auto]
    pub id: Uuid,
    #[index]
    asset_id: Uuid,
    pub l18n: String,

    pub script_type: Script,
    pub level: u8,
    pub school: School,

    pub materials: Materials,
    pub range: Range,
    pub casting: Casting,
    pub duration: Duration,

    #[belongs_to(key = asset_id, references = id)]
    pub script: Asset,

    #[auto]
    pub updated_at: Timestamp,
    #[auto]
    pub created_at: Timestamp,
}
