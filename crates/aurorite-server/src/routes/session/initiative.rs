use axum::extract::{Path, State};
use axum::http::StatusCode;
use crate::state::AuroriteState;
use axum::Router;
use axum::routing::get;
use aurorite_util::uuid::EncodedUuid;
use crate::responses::{AuroriteErrorResponse, FailableResponse, SessionInitiative};
use crate::traits::IntoJson;

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

pub fn build_initiative_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", get(get_initiative))
}