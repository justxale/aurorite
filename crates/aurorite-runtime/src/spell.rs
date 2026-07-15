use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use aurorite_dataflow::database::{Casting, Duration, Materials, Range, School, Script};
use aurorite_dataflow::dto::SpellDto;
use vismut_core::VismutScript;
use crate::RuntimeCtx;

#[derive(Default)]
pub enum CachedScript {
    #[default]
    None,
    Vismut(VismutScript<Arc<Mutex<RuntimeCtx>>>)
}

impl Debug for CachedScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CachedScript::None => write!(f, "CachedScript::None"),
            CachedScript::Vismut(_) => write!(f, "CachedScript::Vismut"),
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Spell {
    pub id: Uuid,
    pub i18n: String,
    pub level: u8,
    pub order: u16,
    pub school: School,
    pub materials: Materials,
    pub range: Range,
    pub casting: Casting,
    pub duration: Duration,
    pub script: Script,
    pub script_asset: String,

    #[serde(skip)]
    #[serde(default)]
    pub cached_script: CachedScript,
}

impl Clone for Spell {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            i18n: self.i18n.clone(),
            level: self.level,
            order: self.order,
            school: self.school,
            materials: self.materials,
            range: self.range,
            casting: self.casting,
            duration: self.duration,
            script: self.script.clone(),
            script_asset: self.script_asset.clone(),
            cached_script: CachedScript::None
        }
    }
}

impl From<SpellDto> for Spell {
    fn from(value: SpellDto) -> Self {
        Self {
            id: value.id,
            i18n: value.i18n,
            level: value.level,
            order: value.order,
            school: value.school,
            materials: value.materials,
            range: value.range,
            casting: value.casting,
            duration: value.duration,
            script: value.script_type,
            script_asset: value.script_asset,
            cached_script: CachedScript::None
        }
    }
}