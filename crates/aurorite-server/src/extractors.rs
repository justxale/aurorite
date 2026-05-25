pub use crate::utils::jwt::Authorization;
use crate::utils::jwt::{KEYS, TokenError};
use axum::RequestPartsExt;
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum_extra::headers;
use axum_extra::headers::authorization::Bearer;
use axum_extra::typed_header::TypedHeader;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use serde::{Deserialize, de::Error};
use crate::database::{Campaign, CampaignClient, Client};
use crate::state::AuroriteState;
use crate::utils::uuid::EncodedUuid;

pub struct AuthorizedClient(pub Client);
pub struct AuthorizedAdmin(pub Client);
pub struct AuthorizedMaster(pub Client, pub Campaign);


impl FromRequestParts<AuroriteState> for AuthorizedClient {
    type Rejection = TokenError;
    async fn from_request_parts(parts: &mut Parts, state: &AuroriteState) -> Result<Self, Self::Rejection> {
        let payload = Authorization::from_request_parts(parts, state).await?;
        let record = Client::get_by_id(&mut state.db(), payload.id()).await.map_err(|_| TokenError::InvalidToken)?;
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
    async fn from_request_parts(parts: &mut Parts, state: &AuroriteState) -> Result<Self, Self::Rejection> {
        let AuthorizedClient(record) = AuthorizedClient::from_request_parts(parts, state).await?;
        if record.is_admin {
            Ok(Self(record))
        } else {
            Err(TokenError::NotAdmin)
        }
    }
}

impl FromRequestParts<AuroriteState> for AuthorizedMaster {
    type Rejection = TokenError;
    async fn from_request_parts(parts: &mut Parts, state: &AuroriteState) -> Result<Self, Self::Rejection> {
        let AuthorizedClient(client) = AuthorizedClient::from_request_parts(parts, state).await?;
        let Path(EncodedUuid(campaign_id)) = Path::<EncodedUuid>::from_request_parts(parts, state)
            .await
            .map_err(|err| TokenError::NotFound(err.to_string()))?;
        if client.campaigns.is_unloaded() {
            return Err(TokenError::Failed("campaigns not loaded".into()));
        }
        let record = CampaignClient::get_by_client_id_and_campaign_id(&mut state.db(), client.id, campaign_id)
            .await
            .map_err(|_| TokenError::NotFound(format!("campaign {} not found", campaign_id)))?;
        if client.is_admin || record.is_master {
            Ok(Self(client, record.campaign.get().clone()))
        } else {
            Err(TokenError::NotMaster)
        }
    }
}
