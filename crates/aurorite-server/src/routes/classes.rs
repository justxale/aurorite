use aurorite_dataflow::database::Class;
use crate::extractors::{AuthorizedAdmin, AuthorizedClient};
use crate::requests::PostClass;
use crate::responses::{AllClassesInfo, AuroriteErrorResponse, ClassInfo, FailableResponse};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use aurorite_util::uuid::EncodedUuid;
use axum::Router;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::routing::get;

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
            classes: records.into_iter().map(|v| ClassInfo::from(&v)).collect(),
        }
        .json(),
    ))
}

async fn post_class(
    State(state): State<AuroriteState>,
    AuthorizedAdmin(_client): AuthorizedAdmin,
    Json(body): Json<PostClass>,
) -> FailableResponse<ClassInfo> {
    let record = Class::create()
        .l18n_key(body.l18n)
        .base_hits(body.base_hits)
        .base_hit_dice(body.base_hit_dice)
        .dyn_data(body.dyn_data)
        .exec(&mut state.db())
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                AuroriteErrorResponse::new(err).json(),
            )
        })?;
    Ok((StatusCode::CREATED, ClassInfo::from(&record).json()))
}

async fn get_class(
    State(state): State<AuroriteState>,
    AuthorizedClient(_client): AuthorizedClient,
    Path(EncodedUuid(class_id)): Path<EncodedUuid>,
) -> FailableResponse<ClassInfo> {
    let record = Class::get_by_id(&mut state.db(), class_id)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                AuroriteErrorResponse::new(err).json(),
            )
        })?;
    Ok((StatusCode::OK, ClassInfo::from(&record).json()))
}

pub fn build_classes_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", get(get_classes).post(post_class))
        .route("/{class_id}", get(get_class))
}
