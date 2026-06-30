use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::database::{Background, BackgroundData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundDto {
    id: Uuid,
    l18n_key: String,
    dynamic: Option<BackgroundData>,
}

impl From<&Background> for BackgroundDto {
    fn from(background: &Background) -> Self {
        Self {
            id: background.id,
            l18n_key: background.l18n_key.clone(),
            dynamic: background.dyn_data.as_ref().map(|v| v.0.clone()),
        }
    }
}