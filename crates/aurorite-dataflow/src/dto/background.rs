use serde::Serialize;
use uuid::Uuid;
use crate::database::{Background, BackgroundData};

#[derive(Debug, Clone, Serialize)]
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
            dynamic: background.dyn_data.clone(),
        }
    }
}