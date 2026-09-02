use crate::responses::SessionClientInfo;
use axum::extract::ws::Message;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use aurorite_runtime::{RuntimeEvent, ThrowEntry};
use aurorite_util::uuid::EncodedUuid;

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
#[serde(tag = "event", content = "payload", rename_all = "snake_case")]
pub enum WebsocketMessage {
    Auth {
        token: String,
    },
    
    OnMessage {
        client: SessionClientInfo,
        content: String,
        created_at: Timestamp,
    },
    OnShutdown {
        reason: Option<String>,
    },

    // runtime events
    LoadInitiative,
    UnloadInitiative,
    LoadScene(EncodedUuid),
    UnloadScene,
    
    OnDiceThrow(ThrowEntry),
    OnDicesThrow(Vec<ThrowEntry>)
}

impl TryFrom<&Message> for WebsocketMessage {
    type Error = WebsocketError;
    fn try_from(message: &Message) -> Result<Self, Self::Error> {
        serde_json::from_str::<Self>(
            message
                .to_text()
                .map_err(|_| WebsocketError::InvalidEncoding)?,
        )
        .map_err(|_| WebsocketError::InvalidSchema)
    }
}

impl From<RuntimeEvent> for WebsocketMessage {
    fn from(event: RuntimeEvent) -> Self {
        match event {
            RuntimeEvent::ThrowDice(throw) => WebsocketMessage::OnDiceThrow(throw),
            RuntimeEvent::ThrowDices(throws) => WebsocketMessage::OnDicesThrow(throws),

            RuntimeEvent::LoadInitiative => WebsocketMessage::LoadInitiative,
            RuntimeEvent::UnloadInitiative => WebsocketMessage::UnloadInitiative,

            RuntimeEvent::LoadScene(id) => WebsocketMessage::LoadScene(id),
            RuntimeEvent::UnloadScene => WebsocketMessage::UnloadScene,

        }
    }
}