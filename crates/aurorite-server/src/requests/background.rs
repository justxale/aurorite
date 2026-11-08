use aurorite_dataflow::database::BackgroundData;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct PostBackground {
    pub i18n: String,
    #[serde(flatten)]
    #[schema(value_type = Option<serde_json::Value>)]
    pub dynamic: Option<BackgroundData>,
}
