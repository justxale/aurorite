use aurorite_dataflow::dto::BackgroundDto;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AllBackgroundsInfo {
    pub backgrounds: Vec<BackgroundDto>,
}
