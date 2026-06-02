use aurorite_dataflow::database::{Campaign, CampaignClient, Client};
use crate::state::AuroriteState;
use aurorite_util::jwt::{TokenError, Authorization, decode_key};
use aurorite_util::uuid::EncodedUuid;
use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, OptionalFromRequestParts, Path, Json};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum_extra::headers;
use axum_extra::headers::authorization::Bearer;
use axum_extra::typed_header::TypedHeader;
use crate::responses::AuroriteErrorResponse;
use crate::traits::IntoJson;

pub struct AuthorizedClient(pub Client);
pub struct AuthorizedAdmin(pub Client);
pub struct AuthorizedMaster<const LOAD_CAMPAIGN: bool>(pub Client, pub Campaign);
pub struct AuthorizedUnchecked(pub Authorization);

impl<S> FromRequestParts<S> for AuthorizedUnchecked
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<AuroriteErrorResponse>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(headers::Authorization(bearer)) = parts
            .extract::<TypedHeader<headers::Authorization<Bearer>>>()
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, AuroriteErrorResponse::new(TokenError::InvalidToken).json()))?;
        Ok(Self(
            decode_key(bearer.token()).map_err(|e| (StatusCode::UNAUTHORIZED, AuroriteErrorResponse::new(e).json()))?
        ))
    }
}

impl FromRequestParts<AuroriteState> for AuthorizedClient {
    type Rejection = (StatusCode, Json<AuroriteErrorResponse>);
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AuroriteState,
    ) -> Result<Self, Self::Rejection> {
        let AuthorizedUnchecked(payload) = AuthorizedUnchecked::from_request_parts(parts, state).await?;
        let record = Client::get_by_id(&mut state.db(), payload.id())
            .await
            .map_err(|_| (StatusCode::UNAUTHORIZED, AuroriteErrorResponse::new(TokenError::InvalidToken).json()))?;
        Ok(Self(record))
    }
}

impl FromRequestParts<AuroriteState> for AuthorizedAdmin {
    type Rejection = (StatusCode, Json<AuroriteErrorResponse>);
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AuroriteState,
    ) -> Result<Self, Self::Rejection> {
        let AuthorizedClient(record) = AuthorizedClient::from_request_parts(parts, state).await?;
        if record.is_admin {
            Ok(Self(record))
        } else {
            Err((StatusCode::FORBIDDEN, AuroriteErrorResponse::new(TokenError::NotAdmin).json()))
        }
    }
}

impl OptionalFromRequestParts<AuroriteState> for AuthorizedAdmin {
    type Rejection = (StatusCode, Json<AuroriteErrorResponse>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AuroriteState,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <AuthorizedAdmin as FromRequestParts<AuroriteState>>::from_request_parts(parts, state)
            .await
        {
            Ok(res) => Ok(Some(res)),
            Err(_) => Ok(None),
        }
    }
}

impl<const LOAD_CAMPAIGN: bool> FromRequestParts<AuroriteState>
    for AuthorizedMaster<LOAD_CAMPAIGN>
{
    type Rejection = (StatusCode, Json<AuroriteErrorResponse>);
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AuroriteState,
    ) -> Result<Self, Self::Rejection> {
        let AuthorizedClient(client) = AuthorizedClient::from_request_parts(parts, state).await?;
        let Path(EncodedUuid(campaign_id)) = <Path<EncodedUuid> as FromRequestParts<
            AuroriteState,
        >>::from_request_parts(parts, state)
        .await
        .map_err(|err| (StatusCode::NOT_FOUND, AuroriteErrorResponse::new(TokenError::NotFound(err.to_string())).json()))?;
        let mut db = state.db();
        let record = match LOAD_CAMPAIGN {
            true => CampaignClient::filter_by_client_id_and_campaign_id(client.id, campaign_id)
                .include(CampaignClient::fields().campaign())
                .include(CampaignClient::fields().campaign().classes())
                .include(CampaignClient::fields().campaign().races())
                .include(CampaignClient::fields().campaign().clients()),
            false => CampaignClient::filter_by_client_id_and_campaign_id(
                client.id,
                campaign_id,
            )
        };
        let record = record
            .get(&mut db)
            .await
            .map_err(|_| {(
                StatusCode::NOT_FOUND,
                AuroriteErrorResponse::new(TokenError::NotFound(format!("campaign {} not found", campaign_id))).json()
            )})?;

        if client.is_admin || record.is_master {
            Ok(Self(client, record.campaign.get().clone()))
        } else {
            Err((StatusCode::FORBIDDEN, AuroriteErrorResponse::new(TokenError::NotMaster).json()))
        }
    }
}

impl<const LOAD_CAMPAIGN: bool> OptionalFromRequestParts<AuroriteState>
    for AuthorizedMaster<LOAD_CAMPAIGN>
{
    type Rejection = (StatusCode, Json<AuroriteErrorResponse>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AuroriteState,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <AuthorizedMaster<LOAD_CAMPAIGN> as FromRequestParts<AuroriteState>>::from_request_parts(parts, state).await {
            Ok(res) => Ok(Some(res)),
            Err(_) => Ok(None)
        }
    }
}
