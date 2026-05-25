use crate::database::Campaign;
use crate::extractors::Authorization;
use crate::requests::PostCampaign;
use crate::responses::{AuroriteErrorResponse, CampaignInfo, FailableResponse};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use uuid::Uuid;

async fn get_campaign(
    Path(id): Path<Uuid>,
    State(state): State<AuroriteState>,
    user: Authorization,
) -> FailableResponse<CampaignInfo> {
    let mut db = state.db();
    match Campaign::filter_by_id(id)
        .include(Campaign::fields().clients())
        .get(&mut db).await
    {
        Err(err) => Err((StatusCode::NOT_FOUND, AuroriteErrorResponse::new(err).json())),
        Ok(ref res) => match CampaignInfo::try_from(res) {
            Ok(res) => Ok((StatusCode::OK, res.json())),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new("failed to build response").json()))
        }
    }
}

async fn post_campaign(
    State(state): State<AuroriteState>,
    user: Authorization,
    Json(body): Json<PostCampaign>,
) -> FailableResponse<CampaignInfo> {
    let record = Campaign::create()
        .title(body.title)
        .owner_id(user.id());
    let mut db = state.db();
    match record.exec(&mut db).await {
        Err(err) => Err((StatusCode::NOT_FOUND, AuroriteErrorResponse::new(err).json())),
        Ok(ref res) => match CampaignInfo::try_from(res) {
            Ok(res) => Ok((StatusCode::OK, res.json())),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new("failed to build response").json()))
        }
    }
}

pub fn build_campaign_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", post(post_campaign))
        .route("/{id}", get(get_campaign))
}