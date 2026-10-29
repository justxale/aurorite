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
use axum::extract::{DefaultBodyLimit, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(
    get, path = "/tracks",
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
    post, path = "/tracks",
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
    delete, path = "/tracks/{id}",
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
        .routes(routes!(delete_track))
        .layer(DefaultBodyLimit::max(64 * 1024usize.pow(3)))
}
