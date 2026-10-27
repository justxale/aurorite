use crate::extractors::{AuroriteMultipart, AuthorizedClient};
use crate::requests::MpPostTrack;
use crate::responses::{AuroriteErrorResponse, ClientTracks, FailableResponse};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_agsp::consts::track_path;
use aurorite_dataflow::database::Track;
use aurorite_dataflow::dto::TrackDto;
use aurorite_util::uuid::EncodedUuid;
use axum::Router;
use axum::extract::{DefaultBodyLimit, State};
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::routing::{RouterExt, TypedPath};
use serde::Deserialize;
use aurorite_agsp::fs::delete_file;

#[derive(TypedPath, Deserialize)]
#[typed_path("/tracks/{id}")]
struct TrackIdPath {
    id: EncodedUuid
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/tracks")]
struct TracksPath;

async fn get_tracks(
    _: TracksPath,
    AuthorizedClient(client): AuthorizedClient,
    State(state): State<AuroriteState>
) -> FailableResponse<ClientTracks> {
    match Track::filter_by_client_id(client.id).exec(&mut state.db()).await {
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(e).json())),
        Ok(records) => Ok((StatusCode::OK, ClientTracks {
            total: records.len(),
            tracks: records.into_iter().filter_map(|v| TrackDto::try_from(v).ok()).collect(),
        }.json()))
    }
}

async fn post_track(
    _: TracksPath,
    AuthorizedClient(client): AuthorizedClient,
    State(state): State<AuroriteState>,
    multipart: AuroriteMultipart<MpPostTrack>
) -> FailableResponse<TrackDto> {
    let record = match Track::create().name(&multipart.name).client(client).exec(&mut state.db()).await {
        Ok(r) => r,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
    };
    match multipart.data.blob.contents.persist(track_path(record.id)) {
        Ok(_) => match TrackDto::try_from(record) {
            Ok(v) => {
                Ok((StatusCode::CREATED, v.json()))
            }
            Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
        }
        Err(err) => {
            Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
        }
    }
}

async fn delete_track(
    TrackIdPath { id }: TrackIdPath,
    AuthorizedClient(client): AuthorizedClient,
    State(state): State<AuroriteState>
) -> Result<StatusCode, impl IntoResponse> {
    let mut db = state.db();
    let record = match Track::filter_by_id(id.uuid()).filter_by_client_id(client.id).get(&mut db).await {
        Ok(r) => r,
        Err(err) => {
            return if err.is_record_not_found() {
                Err(AuroriteErrorResponse::with_code(StatusCode::NOT_FOUND, err))
            } else {
                Err(AuroriteErrorResponse::new(err))
            }
        }
    };
    if let Err(err) = delete_file(track_path(record.id)).await {
        return Err(AuroriteErrorResponse::new(err))
    }
    match record.delete().exec(&mut db).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(AuroriteErrorResponse::new(err))
    }
}

pub fn build_track_routes() -> Router<AuroriteState> {
    Router::new()
        .typed_get(get_tracks)
        .typed_post(post_track.layer(DefaultBodyLimit::max(64 * 1024usize.pow(3))))
        .typed_delete(delete_track)
}