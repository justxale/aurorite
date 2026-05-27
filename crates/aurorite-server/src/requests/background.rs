use crate::database::BackgroundData;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PostBackground {
    pub l18n: String,
    #[serde(flatten)]
    pub dynamic: Option<BackgroundData>,
}
