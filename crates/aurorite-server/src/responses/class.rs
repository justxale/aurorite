use aurorite_dataflow::dto::ClassDto;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AllClassesInfo {
    pub classes: Vec<ClassDto>,
}
