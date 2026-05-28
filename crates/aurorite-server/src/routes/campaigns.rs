use crate::database::{Campaign, CampaignClient};
use crate::extractors::{AuthorizedAdmin, AuthorizedClient, AuthorizedMaster};
use crate::requests::PostCampaign;
use crate::responses::{
    AuroriteErrorResponse, ClientCampaigns, FailableResponse, FullCampaignInfo,
};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use crate::utils::uuid::EncodedUuid;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};

async fn get_campaigns(
    State(state): State<AuroriteState>,
    AuthorizedClient(client): AuthorizedClient,
) -> FailableResponse<ClientCampaigns> {
    let mut db = state.db();
    let campaigns = Campaign::all()
        .filter(
            Campaign::fields()
                .clients()
                .any(CampaignClient::fields().client_id().eq(client.id)),
        )
        .filter(Campaign::fields().is_active().eq(true))
        .exec(&mut db)
        .await;
    match campaigns {
        Ok(ref campaigns) => Ok((StatusCode::OK, ClientCampaigns::from(campaigns).json())),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new(err).json(),
        )),
    }
}

async fn post_campaign(
    State(state): State<AuroriteState>,
    AuthorizedAdmin(client): AuthorizedAdmin,
    Json(body): Json<PostCampaign>,
) -> FailableResponse<FullCampaignInfo> {
    let mut db = state.db();
    let record = Campaign::create()
        .title(body.title)
        .owner_id(client.id)
        .exec(&mut db)
        .await;
    match record {
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new(err).json(),
        )),
        Ok(record) => {
            let _ = record
                .clients()
                .create()
                .campaign(&record)
                .client(client)
                .exec(&mut db)
                .await;
            let res = Campaign::filter_by_id(record.id)
                .include(Campaign::fields().clients())
                .include(Campaign::fields().clients().client())
                .get(&mut db)
                .await;
            match res {
                Ok(ref record) => match FullCampaignInfo::try_from(record) {
                    Ok(res) => Ok((StatusCode::CREATED, res.json())),
                    Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.json())),
                },
                Err(err) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    AuroriteErrorResponse::new(err).json(),
                )),
            }
        }
    }
}

async fn get_campaign(
    State(_state): State<AuroriteState>,
    AuthorizedMaster(_client, campaign): AuthorizedMaster<true>,
) -> FailableResponse<FullCampaignInfo> {
    match FullCampaignInfo::try_from(&campaign) {
        Ok(res) => Ok((StatusCode::OK, res.json())),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new("failed to build response").json(),
        )),
    }
}

async fn delete_campaign(
    Path(EncodedUuid(id)): Path<EncodedUuid>,
    AuthorizedClient(client): AuthorizedClient,
    State(state): State<AuroriteState>,
) -> Result<(StatusCode, ()), (StatusCode, Json<AuroriteErrorResponse>)> {
    let mut db = state.db();
    match Campaign::filter_by_id(id)
        .filter(Campaign::fields().owner_id().eq(client.id))
        .get(&mut db)
        .await
    {
        Err(err) => Err((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new(err).json(),
        )),
        Ok(mut record) => match record.update().is_active(false).exec(&mut db).await {
            Err(err) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                AuroriteErrorResponse::new(err).json(),
            )),
            Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
        },
    }
}

pub fn build_campaign_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", get(get_campaigns).post(post_campaign))
        .route("/{campaign_id}", get(get_campaign).delete(delete_campaign))
}
