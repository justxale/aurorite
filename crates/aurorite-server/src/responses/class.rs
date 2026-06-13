use serde::Serialize;
use aurorite_dataflow::dto::ClassObj;

#[derive(Debug, Serialize)]
pub struct AllClassesInfo {
    pub classes: Vec<ClassObj>,
}
