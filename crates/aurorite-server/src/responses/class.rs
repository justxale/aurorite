use crate::database::{Class, ClassData};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ClassInfo {
    id: Uuid,
    l18n_key: String,
    dynamic: Option<ClassData>,

    base_hits: u16,
    base_hit_dice: String,
}

impl From<&Class> for ClassInfo {
    fn from(class: &Class) -> Self {
        Self {
            id: class.id,
            l18n_key: class.l18n_key.clone(),
            dynamic: class.dyn_data.clone(),
            base_hits: class.base_hits,
            base_hit_dice: class.base_hit_dice.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AllClassesInfo {
    pub classes: Vec<ClassInfo>,
}
