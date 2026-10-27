use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;

#[derive(TryFromMultipart)]
#[try_from_multipart(strict, rename_all = "snake_case")]
pub struct MpPostTrack {
    pub name: String,
    pub tags: Vec<String>,
    #[form_data(limit = "64MB")]
    pub blob: FieldData<NamedTempFile>
}