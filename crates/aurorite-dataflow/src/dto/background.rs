use crate::database::{Background, BackgroundData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundDto {
    id: Uuid,
    i18n: String,
    dynamic: Option<BackgroundData>,
}

impl From<&Background> for BackgroundDto {
    fn from(background: &Background) -> Self {
        Self {
            id: background.id,
            i18n: background.i18n.clone(),
            dynamic: background.dyn_data.as_ref().map(|v| v.0.clone()),
        }
    }
}
