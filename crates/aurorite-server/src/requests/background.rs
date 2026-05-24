use serde::Deserialize;
use crate::database::BackgroundData;

#[derive(Clone, Debug, Deserialize)]
pub struct PostBackground {
    pub l18n: String,
    #[serde(flatten)]
    pub dynamic: Option<BackgroundData>
}