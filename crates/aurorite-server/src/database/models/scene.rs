use crate::database::Asset;
use toasty::{BelongsTo, Model};
use uuid::Uuid;

#[derive(Debug, Clone, Model)]
pub struct Scene {
    #[key]
    pub id: Uuid,
    pub l18n_key: Option<String>,

    #[index]
    asset_id: Option<Uuid>,
    #[belongs_to(key = asset_id, references = id)]
    pub asset: BelongsTo<Option<Asset>>,
}
