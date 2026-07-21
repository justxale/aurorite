use crate::database::Scene;
use crate::database::{CampaignScene, PreloadedObject};
use crate::dto::CharacterDto;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PreloadDto {
    pub is_visible: bool,
    pub character: CharacterDto,
}

impl TryFrom<&PreloadedObject> for PreloadDto {
    type Error = &'static str;
    fn try_from(value: &PreloadedObject) -> Result<Self, Self::Error> {
        if value.character.is_unloaded() {
            return Err("failed to load character data");
        }
        Ok(Self {
            is_visible: value.is_visible,
            character: CharacterDto::try_from(value.character.get())?,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SceneDto {
    pub asset: Option<String>,
    pub preloads: Vec<PreloadDto>,
}

impl TryFrom<&Scene> for SceneDto {
    type Error = &'static str;
    fn try_from(value: &Scene) -> Result<Self, Self::Error> {
        if value.asset.is_unloaded() || value.preloads.is_unloaded() {
            return Err("failed to load scene data");
        }
        let preloads = value
            .preloads
            .get()
            .iter()
            .flat_map(PreloadDto::try_from)
            .collect();
        Ok(Self {
            asset: value.asset.get().as_ref().map(|v| v.filename.clone()),
            preloads,
        })
    }
}

impl TryFrom<&CampaignScene> for SceneDto {
    type Error = &'static str;
    fn try_from(value: &CampaignScene) -> Result<Self, Self::Error> {
        SceneDto::try_from(&value.scene)
    }
}
