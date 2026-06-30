use aurorite_dataflow::database::{Background, ToastyJson};
use crate::extractors::{AuthorizedAdmin, AuthorizedClient};
use crate::requests::PostBackground;
use crate::responses::{
    AllBackgroundsInfo, AuroriteErrorResponse, FailableResponse,
};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_util::uuid::EncodedUuid;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use aurorite_dataflow::dto::BackgroundDto;

async fn get_backgrounds(
    State(state): State<AuroriteState>,
    AuthorizedClient(_client): AuthorizedClient,
) -> FailableResponse<AllBackgroundsInfo> {
    let records = Background::all()
        .exec(&mut state.db())
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                AuroriteErrorResponse::new(err).json(),
            )
        })?;
    Ok((
        StatusCode::OK,
        AllBackgroundsInfo {
            backgrounds: records
                .into_iter()
                .map(|v| BackgroundDto::from(&v))
                .collect(),
        }
        .json(),
    ))
}

async fn post_background(
    State(state): State<AuroriteState>,
    AuthorizedAdmin(_client): AuthorizedAdmin,
    Json(body): Json<PostBackground>,
) -> FailableResponse<BackgroundDto> {
    let record = Background::create()
        .dyn_data(body.dynamic.map(|v| ToastyJson(v)))
        .l18n_key(body.l18n);
    match record.exec(&mut state.db()).await {
        Ok(ref result) => Ok((StatusCode::OK, BackgroundDto::from(result).json())),
        Err(err) => Err((StatusCode::CONFLICT, AuroriteErrorResponse::new(err).json())),
    }
}

async fn get_background(
    Path(EncodedUuid(id)): Path<EncodedUuid>,
    AuthorizedClient(_client): AuthorizedClient,
    State(state): State<AuroriteState>,
) -> FailableResponse<BackgroundDto> {
    match Background::get_by_id(&mut state.db(), id).await {
        Ok(ref record) => Ok((StatusCode::OK, BackgroundDto::from(record).json())),
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

pub fn build_backgrounds_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", get(get_backgrounds).post(post_background))
        .route("/{id}", get(get_background))
}
