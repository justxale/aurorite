use aurorite_dataflow::database::BackgroundData;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct PostBackground {
    pub i18n: String,
    #[serde(flatten)]
    pub dynamic: Option<BackgroundData>,
}
