use aurorite_dataflow::database::{AccessState, Campaign, CampaignClient, Visibility};
use crate::extractors::{AuthorizedAdmin, AuthorizedClient, AuthorizedMaster};
use crate::requests::{PostCampaign, PutCampaignSession};
use crate::responses::{AuroriteErrorResponse, ClientCampaigns, FailableResponse};
use crate::state::AuroriteState;
use crate::traits::IntoJson;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use aurorite_dataflow::dto::CampaignDto;

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
) -> FailableResponse<CampaignDto> {
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
                .include(Campaign::fields().scene())
                .include(Campaign::fields().clients().client())
                .include(Campaign::fields().scenes())
                .get(&mut db)
                .await;
            match res {
                Ok(record) => match CampaignDto::try_from(record) {
                    Ok(res) => Ok((StatusCode::CREATED, res.json())),
                    Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json())),
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
) -> FailableResponse<CampaignDto> {
    match CampaignDto::try_from(campaign) {
        Ok(res) => Ok((StatusCode::OK, res.json())),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new("failed to build response").json(),
        )),
    }
}

async fn delete_campaign(
    AuthorizedMaster(_client, mut campaign): AuthorizedMaster<false>,
    State(state): State<AuroriteState>,
) -> Result<(StatusCode, ()), (StatusCode, Json<AuroriteErrorResponse>)> {
    match campaign.update().is_active(false).exec(&mut state.db()).await {
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            AuroriteErrorResponse::new(err).json(),
        )),
        Ok(_) => Ok((StatusCode::NO_CONTENT, ())),
    }
}

async fn get_campaign_session(
    AuthorizedMaster(_client, campaign): AuthorizedMaster<false>,
) -> FailableResponse<AccessState> {
    Ok((StatusCode::OK, campaign.access_state.json()))
}

async fn put_campaign_session(
    AuthorizedMaster(_client, mut campaign): AuthorizedMaster<false>,
    State(state): State<AuroriteState>,
    Json(body): Json<PutCampaignSession>,
) -> FailableResponse<AccessState> {
    let record = match body.visibility {
        Visibility::Private => campaign.update().access_state(AccessState::private()),
        Visibility::InviteOnly => campaign.update().access_state(AccessState::invite_only())
    };
    match record.exec(&mut state.db()).await {
        Ok(_) => {
            match body.visibility {
                Visibility::Private => state.manager.detach(campaign.id),
                Visibility::InviteOnly => state.manager.attach(campaign.id),
            };
            Ok((StatusCode::OK, campaign.access_state.json()))
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, AuroriteErrorResponse::new(err).json())),
    }
}

pub fn build_campaign_routes() -> Router<AuroriteState> {
    Router::new()
        .route("/", get(get_campaigns).post(post_campaign))
        .route("/{campaign_id}", get(get_campaign).delete(delete_campaign))
        .route("/{campaign_id}/session", get(get_campaign_session).put(put_campaign_session))
}
