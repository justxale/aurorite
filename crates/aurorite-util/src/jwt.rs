use crate::env;
use crate::uuid::serde_support;
use jiff::ToSpan;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::{error, fmt};

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| Keys::new(env().secret.as_bytes()));
const TOKEN_LIFETIME: i8 = 24;

#[derive(Debug)]
pub enum TokenError {
    InvalidToken,
    Failed(String),
    MissingToken,
    NotAdmin,
    NotMaster,
    NotFound(String),
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::Failed(msg) => write!(f, "failure: {}", msg),
            TokenError::NotFound(msg) => write!(f, "{} not found", msg),
            TokenError::InvalidToken => write!(f, "invalid token"),
            TokenError::MissingToken => write!(f, "missing token"),
            TokenError::NotAdmin => write!(f, "not admin"),
            TokenError::NotMaster => write!(f, "not master"),
        }
    }
}

impl error::Error for TokenError {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorization {
    #[serde(with = "serde_support")]
    sub: uuid::Uuid,
    exp: usize,
}

impl Authorization {
    pub fn new(sub: uuid::Uuid) -> Self {
        let exp = jiff::Timestamp::now() + TOKEN_LIFETIME.hours();
        Self {
            sub,
            exp: exp.as_second() as usize,
        }
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
        .map_err(|_| TokenError::Failed("failed to encode key".into()))
}

pub fn decode_key(token: &str) -> Result<Authorization, TokenError> {
    Ok(
        decode::<Authorization>(token, &KEYS.decoding, &Validation::default())
            .map_err(|_| TokenError::InvalidToken)?
            .claims,
    )
}
