use crate::responses::AuroriteErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::sync::LazyLock;
use std::{error, fmt};
use jiff::ToSpan;
use serde::{Deserialize, Serialize};
use crate::utils::uuid::{deserialize_encoded_uuid, serialize_encoded_uuid};

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("AURORITE_SECRET").expect("AUTH_SECRET");
    Keys::new(secret.as_bytes())
});

#[derive(Debug)]
pub enum TokenError {
    InvalidToken,
    Failed,
    MissingToken,
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            TokenError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            TokenError::Failed => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token"),
            TokenError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing token"),
        };
        (status, Json(AuroriteErrorResponse::new(msg))).into_response()
    }
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl error::Error for TokenError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(deserialize_with = "deserialize_encoded_uuid")]
    #[serde(serialize_with = "serialize_encoded_uuid")]
    sub: uuid::Uuid,
    exp: usize,
}

impl Authorization {
    pub fn new(sub: uuid::Uuid) -> Self {
        let exp = jiff::Timestamp::now() + 1.hours();
        Self { sub, exp: exp.as_second() as usize }
    }
    
    pub fn id(&self) -> &uuid::Uuid {
        &self.sub
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub fn encode_key(sub: uuid::Uuid) -> Result<String, TokenError> {
    let claims = Authorization::new(sub);
    encode::<Authorization>(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| TokenError::Failed)
}

pub fn decode_key(token: &str) -> Result<Authorization, TokenError> {
    Ok(decode::<Authorization>(token, &KEYS.decoding, &Validation::default())
        .map_err(|_| TokenError::InvalidToken)?
        .claims
    )
}