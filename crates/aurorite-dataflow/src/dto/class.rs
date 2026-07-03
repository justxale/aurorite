use crate::database::{Class, ClassData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDto {
    pub id: Uuid,
    pub i18n: String,
    pub dynamic: Option<ClassData>,

    pub base_hits: u16,
    pub base_hit_dice: String,
}

impl From<&Class> for ClassDto {
    fn from(class: &Class) -> Self {
        Self {
            id: class.id,
            i18n: class.i18n.clone(),
            dynamic: class.dyn_data.as_ref().map(|v| v.0.clone()),
            base_hits: class.base_hits,
            base_hit_dice: class.base_hit_dice.clone(),
        }
    }
}
