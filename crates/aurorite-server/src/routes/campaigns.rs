use crate::database::Campaign;
use crate::extractors::{AuthorizedAdmin, AuthorizedMaster};
use crate::requests::PostCampaign;
use crate::responses::{AuroriteErrorResponse, CampaignInfo, FailableResponse};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};

#[axum::debug_handler]
async fn get_campaign(
    State(_state): State<AuroriteState>,
    AuthorizedMaster(_client, campaign): AuthorizedMaster<true>,
) -> FailableResponse<CampaignInfo> {
    match CampaignInfo::try_from(&campaign) {
        Ok(res) => Ok((StatusCode::OK, res.json())),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new("failed to build response").json(),
        )),
    }
}

async fn post_campaign(
    State(state): State<AuroriteState>,
    AuthorizedAdmin(client): AuthorizedAdmin,
    Json(body): Json<PostCampaign>,
) -> FailableResponse<CampaignInfo> {
    let record = Campaign::create().title(body.title).owner_id(client.id);
    match record.exec(&mut state.db()).await {
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new(err).json(),
        )),
        Ok(ref res) => match CampaignInfo::try_from(res) {
            Ok(res) => Ok((StatusCode::OK, res.json())),
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                AuroriteErrorResponse::new("failed to build response").json(),
            )),
        },
    }
}

pub fn build_campaign_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", post(post_campaign))
        .route("/{campaign_id}", get(get_campaign))
}
