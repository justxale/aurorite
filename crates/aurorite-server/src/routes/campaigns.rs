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
    let mut db = state.db();
    let record = Campaign::create()
        .title(body.title)
        .owner_id(client.id)
        .exec(&mut db)
        .await;
    match record {
        Err(err) => Err((
            StatusCode::NOT_FOUND, AuroriteErrorResponse::new(err).json()
        )),
        Ok(record) => {
            let _ = record.clients().create().campaign(&record).client(client).exec(&mut db).await;
            let res = Campaign::filter_by_id(record.id)
                .include(Campaign::fields().clients())
                .include(Campaign::fields().clients().client())
                .get(&mut db).await;
            match res {
                Ok(ref record) => match CampaignInfo::try_from(record) {
                    Ok(res) => Ok((StatusCode::OK, res.json())),
                    Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.json())),
                },
                Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json()))
            }
        }
    }
}

pub fn build_campaign_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", post(post_campaign))
        .route("/{campaign_id}", get(get_campaign))
}
