pub use crate::utils::jwt::Authorization;
use crate::utils::jwt::{KEYS, TokenError};
use axum::RequestPartsExt;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::headers;
use axum_extra::headers::authorization::Bearer;
use axum_extra::typed_header::TypedHeader;
use jsonwebtoken::Validation;
use jsonwebtoken::decode;
use serde::{Deserialize, de::Error};


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
