use jiff::Timestamp;
use serde::Serialize;
use aurorite_util::uuid::EncodedUuid;
use crate::database::{Playlist, PlaylistTrack, Track};

#[derive(Serialize)]
pub struct TrackDto {
    pub id: EncodedUuid,
    pub name: String,
    pub client_id: EncodedUuid,
    pub tags: Vec<String>,
    
    pub order: Option<u16>,
    pub is_looped: bool,
    
    pub updated_at: Timestamp,
    pub created_at: Timestamp,
}

impl TryFrom<Track> for TrackDto {
    type Error = &'static str;

    fn try_from(value: Track) -> Result<Self, Self::Error> {
        if value.tags.is_unloaded() {
            return Err("failed to load tags data")
        }
        Ok(Self {
            id: EncodedUuid(value.id),
            name: value.name,
            is_looped: false,
            order: None,
            client_id: EncodedUuid(value.client_id),
            tags: value.tags.into_inner().into_iter().map(|v| v.tag.value).collect(),
            updated_at: value.updated_at,
            created_at: value.created_at
        })
    }
}

impl TryFrom<PlaylistTrack> for TrackDto {
    type Error = &'static str;
    fn try_from(value: PlaylistTrack) -> Result<Self, Self::Error> {
        let mut dto = TrackDto::try_from(value.track)?;
        dto.is_looped = value.is_looped;
        dto.order = value.order;
        Ok(dto)
    }
}

#[derive(Serialize)]
pub struct PlaylistDto {
    pub id: EncodedUuid,
    pub name: String,
    pub client_id: EncodedUuid,
    pub tracks: Vec<TrackDto>,
    pub updated_at: Timestamp,
    pub created_at: Timestamp,
}

impl TryFrom<Playlist> for PlaylistDto {
    type Error = &'static str;
    fn try_from(value: Playlist) -> Result<Self, Self::Error> {
        if value.tracks.is_unloaded() {
            return Err("failed to load tracks data")
        }
        Ok(Self {
            id: EncodedUuid(value.id),
            name: value.name,
            client_id: EncodedUuid(value.client_id),
            tracks: value.tracks.into_inner().into_iter().filter_map(|v| {
                TrackDto::try_from(v).ok()
            }).collect(),
            updated_at: value.updated_at,
            created_at: value.created_at,
        })
    }
}