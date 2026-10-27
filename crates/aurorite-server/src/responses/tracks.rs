use serde::Serialize;
use aurorite_dataflow::dto::TrackDto;

#[derive(Serialize)]
pub struct ClientTracks {
    pub tracks: Vec<TrackDto>,
    pub total: usize
}