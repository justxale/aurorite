use jiff::Timestamp;
use toasty::{Deferred, Model};
use uuid::Uuid;
use crate::database::Client;

#[derive(Model)]
pub struct Track {
    #[key]
    pub id: Uuid,

    pub name: String,

    #[index]
    pub client_id: Uuid,

    #[belongs_to(key = client_id, references = id)]
    pub client: Deferred<Client>,

    #[auto]
    pub updated_at: Timestamp,
    #[auto]
    pub created_at: Timestamp,

    #[has_many]
    pub tags: Deferred<Vec<TrackTag>>
}

#[derive(Model)]
pub struct Tag {
    #[key]
    #[column(type = varchar(32))]
    pub value: String,
    pub icon: Option<u32>,

    #[has_many]
    pub tracks: Deferred<Vec<TrackTag>>
}

#[derive(Model)]
pub struct TrackTag {
    #[key]
    #[index]
    track_id: Uuid,
    #[key]
    #[index]
    tag_value: String,

    #[belongs_to(key = track_id, references = id)]
    pub track: Deferred<Track>,
    #[belongs_to(key = tag_value, references = value)]
    pub tag: Tag
}

#[derive(Model)]
pub struct PlaylistTrack {
    #[key]
    track_id: Uuid,
    #[key]
    #[index]
    playlist_id: Uuid,

    pub is_looped: bool,
    pub order: Option<u16>,

    #[belongs_to(key = playlist_id, references = id)]
    pub playlist: Deferred<Playlist>,
    #[belongs_to(key = track_id, references = id)]
    pub track: Track
}

#[derive(Model)]
pub struct Playlist {
    #[key]
    pub id: Uuid,
    pub name: String,

    #[index]
    pub client_id: Uuid,

    #[belongs_to(key = client_id, references = id)]
    pub client: Deferred<Client>,

    #[has_many]
    pub tracks: Deferred<Vec<PlaylistTrack>>,

    #[auto]
    pub updated_at: Timestamp,
    #[auto]
    pub created_at: Timestamp,
}