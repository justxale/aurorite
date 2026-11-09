use std::ops::{Bound, RangeBounds};
use crate::extractors::{AuroriteMultipart, AuthorizedClient};
use crate::requests::MpPostTrack;
use crate::responses::{AuroriteErrorResponse, ClientTracks, FailableResponse, TrackInfo};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_agsp::consts::track_path;
use aurorite_agsp::fs::delete_file;
use aurorite_dataflow::database::Track;
use aurorite_dataflow::dto::TrackDto;
use aurorite_util::uuid::EncodedUuid;
use axum::extract::{DefaultBodyLimit, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::TypedHeader;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use axum_extra::headers::Range;
use axum_extra::response::FileStream;
use tokio_util::io::ReaderStream;
use tracing::log::Level::Trace;

#[utoipa::path(
    get, path = "/",
    security(
        ("password" = [])
    ),
    responses(
        (status = OK, body = ClientTracks),
        (status = INTERNAL_SERVER_ERROR, body = AuroriteErrorResponse)
    ),
    tag = "tracks"
)]
async fn get_tracks(
    AuthorizedClient(client): AuthorizedClient,
    State(state): State<AuroriteState>,
) -> FailableResponse<ClientTracks> {
    match Track::filter_by_client_id(client.id)
        .exec(&mut state.db())
        .await
    {
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new(e).json(),
        )),
        Ok(records) => Ok((
            StatusCode::OK,
            ClientTracks {
                total: records.len(),
                tracks: records
                    .into_iter()
                    .filter_map(|v| TrackDto::try_from(v).map(TrackInfo::from).ok())
                    .collect(),
            }
            .json(),
        )),
    }
}

#[utoipa::path(
    post, path = "/",
    request_body(
        content_type = "multipart/form-data", content = MpPostTrack
    ),
    responses(
        (status = CREATED, body = TrackInfo),
        (status = UNSUPPORTED_MEDIA_TYPE, body = AuroriteErrorResponse),
        (status = INTERNAL_SERVER_ERROR, body = AuroriteErrorResponse)
    ),
    security(
        ("password" = [])
    ),
    tag = "tracks"
)]
async fn post_track(
    AuthorizedClient(client): AuthorizedClient,
    State(state): State<AuroriteState>,
    multipart: AuroriteMultipart<MpPostTrack>,
) -> FailableResponse<TrackInfo> {
    if !multipart.blob.contents.starts_with(b"OggS") {
        return Err((StatusCode::UNSUPPORTED_MEDIA_TYPE, AuroriteErrorResponse::new("only ogg files supported").json()))
    }
    let mut db = state.db();
    let record = match Track::create()
        .name(&multipart.name)
        .client(client)
        .exec(&mut db)
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                AuroriteErrorResponse::new(err).json(),
            ));
        }
    };
    let record = match Track::filter_by_id(record.id)
        .include(Track::fields().tags())
        .get(&mut db)
        .await
    {
        Ok(record) => record,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
    };
    let path = track_path(record.id);
    match File::create(&path).await {
        Ok(mut f) => match f.write_all(&multipart.blob.contents).await {
            Ok(_) => match TrackDto::try_from(record) {
                Ok(v) => Ok((StatusCode::CREATED, TrackInfo::from(v).json())),
                Err(err) => {
                    let _ = f.flush().await;
                    drop(f);
                    let _ = tokio::fs::remove_file(&path).await;
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        AuroriteErrorResponse::new(err).json(),
                    ))
                },
            },
            Err(err) => {
                let _ = f.flush().await;
                drop(f);
                let _ = tokio::fs::remove_file(&path).await;
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    AuroriteErrorResponse::new(err).json(),
                ))
            },
        },
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

#[utoipa::path(
    get, path = "/{id}",
    params(("id", description = "ID of track")),
    responses(
        (status = OK, body = TrackInfo),
        (status = NOT_FOUND, body = AuroriteErrorResponse),
        (status = INTERNAL_SERVER_ERROR, body = AuroriteErrorResponse)
    ),
    tag = "tracks"
)]
async fn get_track(
    Path(id): Path<EncodedUuid>,
    State(state): State<AuroriteState>,

) -> FailableResponse<TrackInfo> {
    let record = match Track::filter_by_id(id.uuid())
        .include(Track::fields().tags())
        .get(&mut state.db())
        .await
    {
        Ok(record) => record,
        Err(_) => return Err((StatusCode::NOT_FOUND, AuroriteErrorResponse::new("track not found").json()))
    };
    let record = match TrackDto::try_from(record) {
        Ok(record) => record,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new("track not found").json()))
    };
    Ok((StatusCode::OK, TrackInfo::from(record).json()))
}

#[utoipa::path(
    get, path = "/{id}/stream",
    params(("id", description = "ID of track")),
    responses(
        (status = PARTIAL_CONTENT),
        (status = NOT_FOUND, body = AuroriteErrorResponse),
        (status = INTERNAL_SERVER_ERROR, body = AuroriteErrorResponse)
    ),
    tag = "tracks"
)]
async fn get_track_stream(
    Path(id): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    range: Option<TypedHeader<Range>>
) -> Result<Response, (StatusCode, AuroriteErrorResponse)> {
    const MAX_UNBOUNDED: u64 = 1024 * 1024u64;
    let record = match Track::filter_by_id(id.uuid())
        .include(Track::fields().tags())
        .get(&mut state.db())
        .await
    {
        Ok(record) => record,
        Err(_) => return Err((StatusCode::NOT_FOUND, AuroriteErrorResponse::new("track not found")))
    };
    let path = track_path(record.id);
    let range = match range.map(|v| v.satisfiable_ranges(record.length).next()) {
        Some(Some(range)) => range,
        None | Some(None) => return Err((StatusCode::RANGE_NOT_SATISFIABLE, AuroriteErrorResponse::new("invalid range")))
    };
    let (start, end) = match range {
        (Bound::Included(start), Bound::Included(end)) => (start, end),
        (Bound::Unbounded, Bound::Included(end)) => (record.length - end, end),
        (Bound::Included(start), Bound::Unbounded) if start < MAX_UNBOUNDED => (start, MAX_UNBOUNDED),
        _ => return Err((StatusCode::RANGE_NOT_SATISFIABLE, AuroriteErrorResponse::new("invalid range")))
    };
    FileStream::<ReaderStream<File>>::try_range_response(path, start, end)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err)))
}

#[utoipa::path(
    delete, path = "/{id}",
    params(("id", description = "ID of track")),
    security(
        ("password" = [])
    ),
    responses(
        (status = NO_CONTENT),
        (status = NOT_FOUND, body = AuroriteErrorResponse),
        (status = INTERNAL_SERVER_ERROR, body = AuroriteErrorResponse)
    ),
    tag = "tracks"
)]
async fn delete_track(
    Path(id): Path<EncodedUuid>,
    AuthorizedClient(client): AuthorizedClient,
    State(state): State<AuroriteState>,
) -> Result<StatusCode, impl IntoResponse> {
    let mut db = state.db();
    let record = match Track::filter_by_id(id.uuid())
        .filter_by_client_id(client.id)
        .get(&mut db)
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return if err.is_record_not_found() {
                Err(AuroriteErrorResponse::with_code(StatusCode::NOT_FOUND, err))
            } else {
                Err(AuroriteErrorResponse::new(err))
            };
        }
    };
    if let Err(err) = delete_file(track_path(record.id)).await {
        return Err(AuroriteErrorResponse::new(err));
    }
    match record.delete().exec(&mut db).await {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(AuroriteErrorResponse::new(err)),
    }
}

pub fn build_track_routes() -> OpenApiRouter<AuroriteState> {
    OpenApiRouter::new()
        .routes(routes!(post_track, get_tracks))
        .routes(routes!(get_track, delete_track))
        .routes(routes!(get_track_stream))
        .layer(DefaultBodyLimit::max(64 * 1024usize.pow(3)))
}
