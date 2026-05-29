use crate::database::Background;
use crate::extractors::AuthorizedClient;
use crate::requests::PostBackground;
use crate::responses::{AuroriteErrorResponse, FailableResponse, BackgroundInfo};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use uuid::fmt::Simple;

async fn get_background(
    Path(id): Path<Simple>,
    AuthorizedClient(_client): AuthorizedClient,
    State(state): State<AuroriteState>,
) -> FailableResponse<BackgroundInfo> {
    match Background::get_by_id(&mut state.db(), id.as_uuid()).await {
        Ok(ref record) => Ok((StatusCode::OK, BackgroundInfo::from(record).json())),
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

async fn post_background(
    State(state): State<AuroriteState>,
    AuthorizedClient(_client): AuthorizedClient,
    Json(body): Json<PostBackground>,
) -> FailableResponse<BackgroundInfo> {
    let record = Background::create()
        .dyn_data(body.dynamic)
        .l18n_key(body.l18n);
    match record.exec(&mut state.db()).await {
        Ok(ref result) => Ok((StatusCode::OK, BackgroundInfo::from(result).json())),
        Err(err) => Err((StatusCode::CONFLICT, AuroriteErrorResponse::new(err).json())),
    }
}

pub fn build_backgrounds_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", post(post_background))
        .route("/{id}", get(get_background))
}
