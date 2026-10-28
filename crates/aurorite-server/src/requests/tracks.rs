use axum::body::Bytes;
use axum_typed_multipart::{FieldData, TryFromMultipart};
use tempfile::NamedTempFile;
use utoipa::ToSchema;

#[derive(TryFromMultipart, ToSchema)]
pub struct MpPostTrack {
    pub name: String,
    pub tags: Vec<String>,
    #[schema(value_type = String, format = Binary)]
    #[form_data(limit = "64MiB")]
    pub blob: FieldData<Bytes>
}