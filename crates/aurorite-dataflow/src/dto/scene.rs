use crate::database::PreloadedObject;
use crate::database::Scene;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PreloadDto {
    pub is_visible: bool,
    pub character: Uuid,
}

impl From<&PreloadedObject> for PreloadDto {
    fn from(dto: &PreloadedObject) -> Self {
        Self {
            is_visible: dto.is_visible,
            character: dto.character_id,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneDto {
    asset: Option<String>,
    preloads: Vec<PreloadDto>,
}

impl TryFrom<&Scene> for SceneDto {
    type Error = &'static str;
    fn try_from(value: &Scene) -> Result<Self, Self::Error> {
        if value.asset.is_unloaded() || value.preloads.is_unloaded() {
            return Err("failed to load data");
        }
        let preloads = value.preloads.get().iter().map(PreloadDto::from).collect();
        Ok(Self {
            asset: value.asset.get().as_ref().map(|v| v.filename.clone()),
            preloads,
        })
    }
}
