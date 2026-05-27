use crate::database::{Campaign, CampaignClient, Client};
use crate::state::AuroriteState;
pub use crate::utils::jwt::Authorization;
use crate::utils::jwt::{KEYS, TokenError};
use crate::utils::uuid::EncodedUuid;
use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, OptionalFromRequestParts, Path};
use axum::http::request::Parts;
use axum_extra::headers;
use axum_extra::headers::authorization::Bearer;
use axum_extra::typed_header::TypedHeader;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;

pub struct AuthorizedClient(pub Client);
pub struct AuthorizedAdmin(pub Client);
pub struct AuthorizedMaster<const LOAD_CAMPAIGN: bool>(pub Client, pub Campaign);

impl FromRequestParts<AuroriteState> for AuthorizedClient {
    type Rejection = TokenError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AuroriteState,
    ) -> Result<Self, Self::Rejection> {
        let payload = Authorization::from_request_parts(parts, state).await?;
        let record = Client::get_by_id(&mut state.db(), payload.id())
            .await
            .map_err(|_| TokenError::InvalidToken)?;
        Ok(Self(record))
    }
}

impl<S> FromRequestParts<S> for Authorization
where
    S: Send + Sync,
{
    type Rejection = TokenError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(headers::Authorization(bearer)) = parts
            .extract::<TypedHeader<headers::Authorization<Bearer>>>()
            .await
            .map_err(|_| TokenError::InvalidToken)?;
        let payload =
            decode::<Authorization>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| TokenError::InvalidToken)?;
        Ok(payload.claims)
    }
}

impl FromRequestParts<AuroriteState> for AuthorizedAdmin {
    type Rejection = TokenError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AuroriteState,
    ) -> Result<Self, Self::Rejection> {
        let AuthorizedClient(record) = AuthorizedClient::from_request_parts(parts, state).await?;
        if record.is_admin {
            Ok(Self(record))
        } else {
            Err(TokenError::NotAdmin)
        }
    }
}

impl OptionalFromRequestParts<AuroriteState> for AuthorizedAdmin {
    type Rejection = TokenError;

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
    type Rejection = TokenError;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AuroriteState,
    ) -> Result<Self, Self::Rejection> {
        let AuthorizedClient(client) = AuthorizedClient::from_request_parts(parts, state).await?;
        let Path(EncodedUuid(campaign_id)) = <Path<EncodedUuid> as FromRequestParts<
            AuroriteState,
        >>::from_request_parts(parts, state)
        .await
        .map_err(|err| TokenError::NotFound(err.to_string()))?;
        let mut db = state.db();
        match LOAD_CAMPAIGN {
            true => {
                let record =
                    CampaignClient::filter_by_client_id_and_campaign_id(client.id, campaign_id)
                        .include(CampaignClient::fields().campaign())
                        .include(CampaignClient::fields().campaign().classes())
                        .include(CampaignClient::fields().campaign().races())
                        .include(CampaignClient::fields().campaign().clients())
                        .get(&mut db)
                        .await
                        .map_err(|_| {
                            TokenError::NotFound(format!("campaign {} not found", campaign_id))
                        })?;
                if client.is_admin || record.is_master {
                    Ok(Self(client, record.campaign.get().clone()))
                } else {
                    Err(TokenError::NotMaster)
                }
            }
            false => {
                let record = CampaignClient::get_by_client_id_and_campaign_id(
                    &mut db,
                    client.id,
                    campaign_id,
                )
                .await
                .map_err(|_| TokenError::NotFound(format!("campaign {} not found", campaign_id)))?;
                if client.is_admin || record.is_master {
                    Ok(Self(client, record.campaign.get().clone()))
                } else {
                    Err(TokenError::NotMaster)
                }
            }
        }
    }
}

impl<const LOAD_CAMPAIGN: bool> OptionalFromRequestParts<AuroriteState>
    for AuthorizedMaster<LOAD_CAMPAIGN>
{
    type Rejection = TokenError;

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
