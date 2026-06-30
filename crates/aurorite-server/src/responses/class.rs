use serde::Serialize;
use aurorite_dataflow::dto::ClassDto;

#[derive(Debug, Serialize)]
pub struct AllClassesInfo {
    pub classes: Vec<ClassDto>,
}
