use jiff::Timestamp;
use serde::Serialize;
use aurorite_dataflow::dto::TrackDto;
use aurorite_util::uuid::EncodedUuid;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct TrackInfo {
    #[schema(value_type = String)]
    pub id: EncodedUuid,
    pub name: String,
    #[schema(value_type = String)]
    pub client_id: EncodedUuid,
    pub tags: Vec<String>,

    pub order: Option<u16>,
    pub is_looped: bool,

    pub updated_at: Timestamp,
    pub created_at: Timestamp,
}

impl From<TrackDto> for TrackInfo {
    fn from(TrackDto {id, name, client_id, tags, order, is_looped, updated_at, created_at}: TrackDto) -> Self {
        Self {
            id, name, client_id, tags, order, is_looped, updated_at, created_at
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ClientTracks {
    pub tracks: Vec<TrackInfo>,
    pub total: usize
}