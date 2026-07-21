use crate::responses::SessionClientInfo;
use std::fmt::Display;
use axum::extract::ws::Message;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum WebsocketError {
    InvalidSchema,
    InvalidEncoding,
}

impl Display for WebsocketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebsocketError::InvalidSchema => write!(f, "invalid schema"),
            WebsocketError::InvalidEncoding => write!(f, "invalid encoding"),
        }
    }
}

impl std::error::Error for WebsocketError {}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum WebsocketMessage {
    Auth { token: String },
    Chat { client: SessionClientInfo, content: String, created_at: Timestamp },
    Shutdown { reason: Option<String> }
}

impl TryFrom<&Message> for WebsocketMessage {
    type Error = WebsocketError;
    fn try_from(message: &Message) -> Result<Self, Self::Error> {
        serde_json::from_str::<Self>(
            message.to_text().map_err(|_| WebsocketError::InvalidEncoding)?
        ).map_err(|_| WebsocketError::InvalidSchema)
    }
}