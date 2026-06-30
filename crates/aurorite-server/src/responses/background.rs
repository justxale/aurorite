use serde::Serialize;
use aurorite_dataflow::dto::BackgroundDto;

#[derive(Debug, Clone, Serialize)]
pub struct AllBackgroundsInfo {
    pub backgrounds: Vec<BackgroundDto>,
}
