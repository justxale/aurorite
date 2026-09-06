use axum::extract::{Path, State};
use axum::http::StatusCode;
use crate::state::AuroriteState;
use axum::{Json, Router};
use axum::routing::get;
use aurorite_util::uuid::EncodedUuid;
use crate::requests::PostSessionInitiative;
use crate::responses::{AuroriteErrorResponse, FailableResponse, SessionInitiative};
use crate::traits::IntoJson;
use uuid::Uuid;

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

pub fn build_initiative_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", get(get_initiative).post(post_initiative).delete(delete_initiative))
}