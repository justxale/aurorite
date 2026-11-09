use jiff::Timestamp;
use serde::Serialize;
use aurorite_util::uuid::EncodedUuid;
use crate::database::{Playlist, PlaylistTrack, Timeline, TimelineClip, Track};

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

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TimelineClipDto {
    Playlist { order: u32, #[serde(flatten)] dto: PlaylistDto },
    Track { order: u32, #[serde(flatten)] dto: TrackDto }
}

impl TryFrom<TimelineClip> for TimelineClipDto {
    type Error = &'static str;
    fn try_from(value: TimelineClip) -> Result<Self, Self::Error> {
        match (value.playlist_id, value.track_id) {
            (Some(_), None) => {
                if value.playlist.is_unloaded() {
                    return Err("failed to load playlist data")
                }
                match value.playlist.into_inner().map(PlaylistDto::try_from) {
                    Some(Ok(dto)) => Ok(Self::Playlist { order: value.order, dto }),
                    Some(Err(err)) => Err(err),
                    None => Err("invalid playlist record")
                }
            },
            (None, Some(_)) => {
                if value.track.is_unloaded() {
                    return Err("failed to load track data")
                }
                match value.track.into_inner().map(TrackDto::try_from) {
                    Some(Ok(dto)) => Ok(Self::Track { order: value.order, dto }),
                    Some(Err(err)) => Err(err),
                    None => Err("invalid track record")
                }
            },
            (None, None) | (Some(_), Some(_)) => Err("invalid clip record")
        }
    }
}

#[derive(Serialize)]
pub struct TimelineDto {
    pub id: EncodedUuid,
    pub clips: Vec<TimelineClipDto>,

    pub updated_at: Timestamp,
    pub created_at: Timestamp,
}

impl TryFrom<Timeline> for TimelineDto {
    type Error = &'static str;

    fn try_from(value: Timeline) -> Result<Self, Self::Error> {
        if value.clips.is_unloaded() {
            return Err("failed to load clips data")
        }
        let clips: Vec<_> = value
            .clips
            .into_inner()
            .into_iter()
            .filter_map(|v| {
                match TimelineClipDto::try_from(v) {
                    Ok(v) => Some(v),
                    Err(err) => {
                        None
                    }
                }
            })
            .collect();
        Ok(Self {
            id: EncodedUuid(value.id),
            clips,
            updated_at: value.updated_at,
            created_at: value.created_at
        })

    }
}