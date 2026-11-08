use axum::extract::{Path, State};
use axum::http::StatusCode;
use crate::state::AuroriteState;
use axum::Json;
use aurorite_util::uuid::EncodedUuid;
use crate::requests::PostSessionInitiative;
use crate::responses::{AuroriteErrorResponse, FailableResponse, SessionInitiative};
use crate::traits::IntoJson;
use uuid::Uuid;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(
    get,
    path = "/",
    params(("session_id", description = "ID of session")),
    tag = "sessions",
    responses((status = OK, description = "Active initiative"), (status = NOT_FOUND, body = AuroriteErrorResponse))
)]
async fn get_initiative(
    Path(EncodedUuid(session_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>
) -> FailableResponse<SessionInitiative> {
    state.session_and(session_id, |v| {
        v.initiative()
            .map(|v| (StatusCode::OK, SessionInitiative::from(v).json()))
            .ok_or((StatusCode::NOT_FOUND, AuroriteErrorResponse::new("no active initiative").json()))
    })?
}

#[utoipa::path(
    post,
    path = "/",
    params(("session_id", description = "ID of session")),
    tag = "sessions",
    security(("password" = [])),
    responses((status = OK, description = "Initiative started"), (status = CONFLICT, body = AuroriteErrorResponse))
)]
async fn post_initiative(
    Path(EncodedUuid(session_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
    Json(request): Json<PostSessionInitiative>
) -> FailableResponse<SessionInitiative> {
    state.session_and(session_id, |v| {
        let ids: Vec<Uuid> = request.members.iter().map(|v| v.uuid()).collect();
        v.load_initiative(&ids)
            .map(|v| (StatusCode::OK, SessionInitiative::from(v).json()))
            .map_err(|v| (StatusCode::CONFLICT, AuroriteErrorResponse::new(v).json()))
    })?
}

#[utoipa::path(
    delete,
    path = "/",
    params(("session_id", description = "ID of session")),
    tag = "sessions",
    security(("password" = [])),
    responses((status = NO_CONTENT, description = "Initiative stopped"), (status = CONFLICT, body = AuroriteErrorResponse))
)]
async fn delete_initiative(
    Path(EncodedUuid(session_id)): Path<EncodedUuid>,
    State(state): State<AuroriteState>,
) -> Result<StatusCode, (StatusCode, Json<AuroriteErrorResponse>)> {
    state.session_and(session_id, |v| {
        v.unload_initiative()
            .map(|_v| StatusCode::NO_CONTENT)
            .map_err(|v| (StatusCode::CONFLICT, AuroriteErrorResponse::new(v).json()))
    })?
}

pub fn build_initiative_routes() -> OpenApiRouter<AuroriteState> {
    OpenApiRouter::new()
        .routes(routes!(get_initiative, post_initiative, delete_initiative))
}