use crate::extractors::{AuthorizedAdmin, AuthorizedClient};
use crate::requests::PostClass;
use crate::responses::{AllClassesInfo, AuroriteErrorResponse, FailableResponse};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_dataflow::database::{Class, ToastyJson};
use aurorite_dataflow::dto::ClassDto;
use aurorite_util::uuid::EncodedUuid;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(
    get,
    path = "/",
    tag = "editions",
    security(("password" = [])),
    responses((status = OK, description = "List classes"), (status = INTERNAL_SERVER_ERROR, body = AuroriteErrorResponse))
)]
async fn get_classes(
    State(state): State<AuroriteState>,
    AuthorizedClient(_client): AuthorizedClient,
) -> FailableResponse<AllClassesInfo> {
    let records = Class::all().exec(&mut state.db()).await.map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new(err).json(),
        )
    })?;
    Ok((
        StatusCode::OK,
        AllClassesInfo {
            classes: records.into_iter().map(|v| ClassDto::from(&v)).collect(),
        }
        .json(),
    ))
}

#[utoipa::path(
    post,
    path = "/",
    tag = "editions",
    security(("password" = [])),
    responses((status = CREATED, description = "Class created"), (status = INTERNAL_SERVER_ERROR, body = AuroriteErrorResponse))
)]
async fn post_class(
    State(state): State<AuroriteState>,
    AuthorizedAdmin(_client): AuthorizedAdmin,
    Json(body): Json<PostClass>,
) -> FailableResponse<ClassDto> {
    let record = Class::create()
        .i18n(body.i18n)
        .base_hits(body.base_hits)
        .base_hit_dice(body.base_hit_dice)
        .dyn_data(body.dyn_data.map(ToastyJson))
        .exec(&mut state.db())
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                AuroriteErrorResponse::new(err).json(),
            )
        })?;
    Ok((StatusCode::CREATED, ClassDto::from(&record).json()))
}

#[utoipa::path(
    get,
    path = "/{class_id}",
    params(("class_id", description = "ID of class")),
    tag = "editions",
    security(("password" = [])),
    responses((status = OK, description = "Class details"), (status = INTERNAL_SERVER_ERROR, body = AuroriteErrorResponse))
)]
async fn get_class(
    State(state): State<AuroriteState>,
    AuthorizedClient(_client): AuthorizedClient,
    Path(EncodedUuid(class_id)): Path<EncodedUuid>,
) -> FailableResponse<ClassDto> {
    let record = Class::get_by_id(&mut state.db(), class_id)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                AuroriteErrorResponse::new(err).json(),
            )
        })?;
    Ok((StatusCode::OK, ClassDto::from(&record).json()))
}

pub fn build_classes_routes() -> OpenApiRouter<AuroriteState> {
    OpenApiRouter::new()
        .routes(routes!(get_classes, post_class))
        .routes(routes!(get_class))
}
