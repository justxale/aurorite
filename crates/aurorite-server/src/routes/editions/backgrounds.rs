use crate::extractors::{AuthorizedAdmin, AuthorizedClient};
use crate::requests::PostBackground;
use crate::responses::{AllBackgroundsInfo, AuroriteErrorResponse, FailableResponse};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_dataflow::database::{Background, ToastyJson};
use aurorite_dataflow::dto::BackgroundDto;
use aurorite_util::uuid::EncodedUuid;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(
    get,
    path = "/",
    tag = "editions",
    security(("password" = [])),
    responses((status = OK, description = "List backgrounds"), (status = INTERNAL_SERVER_ERROR, body = AuroriteErrorResponse))
)]
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

#[utoipa::path(
    post,
    path = "/",
    tag = "editions",
    security(("password" = [])),
    responses((status = OK, description = "Background created"), (status = CONFLICT, body = AuroriteErrorResponse))
)]
async fn post_background(
    State(state): State<AuroriteState>,
    AuthorizedAdmin(_client): AuthorizedAdmin,
    Json(body): Json<PostBackground>,
) -> FailableResponse<BackgroundDto> {
    let record = Background::create()
        .dyn_data(body.dynamic.map(ToastyJson))
        .i18n(body.i18n);
    match record.exec(&mut state.db()).await {
        Ok(ref result) => Ok((StatusCode::OK, BackgroundDto::from(result).json())),
        Err(err) => Err((StatusCode::CONFLICT, AuroriteErrorResponse::new(err).json())),
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(("id", description = "ID of background")),
    tag = "editions",
    security(("password" = [])),
    responses((status = OK, description = "Background details"), (status = NOT_FOUND, body = AuroriteErrorResponse))
)]
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

pub fn build_backgrounds_routes() -> OpenApiRouter<AuroriteState> {
    OpenApiRouter::new()
        .routes(routes!(get_backgrounds, post_background))
        .routes(routes!(get_background))
}
