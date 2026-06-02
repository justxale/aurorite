use aurorite_dataflow::database::{Background, BackgroundData};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct BackgroundInfo {
    id: Uuid,
    l18n_key: String,
    dynamic: Option<BackgroundData>,
}

impl From<&Background> for BackgroundInfo {
    fn from(background: &Background) -> Self {
        Self {
            id: background.id,
            l18n_key: background.l18n_key.clone(),
            dynamic: background.dyn_data.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AllBackgroundsInfo {
    pub backgrounds: Vec<BackgroundInfo>,
}
