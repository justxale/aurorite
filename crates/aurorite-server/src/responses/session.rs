use crate::session::{Session, SessionClient};
use aurorite_runtime::{Character, Initiative};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use aurorite_util::uuid::EncodedUuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SessionClientInfo {
    pub id: Uuid,
    pub name: String,
    pub is_guest: bool,
}

impl SessionClientInfo {
    pub fn from_guest(value: &SessionClient) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            is_guest: true,
        }
    }

    pub fn from_client(value: &SessionClient) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            is_guest: false,
        }
    }
}

#[derive(Serialize)]
pub struct MessageInfo {
    pub client: SessionClientInfo,
    pub content: String,
    pub created_at: Timestamp,
}

#[derive(Serialize)]
pub struct SessionInfo {
    pub clients: Vec<SessionClientInfo>,
    pub started_at: Timestamp,
}

impl From<&Session> for SessionInfo {
    fn from(session: &Session) -> Self {
        let mut clients = Vec::with_capacity(session.clients().len() + session.guests().len());
        session
            .clients()
            .iter()
            .for_each(|client| clients.push(SessionClientInfo::from_client(&client)));
        session
            .guests()
            .iter()
            .for_each(|guest| clients.push(SessionClientInfo::from_guest(&guest)));
        Self {
            clients,
            started_at: session.started_at,
        }
    }
}

#[derive(Serialize)]
pub struct SessionCharacters {
    pub characters: Vec<Character>,
}

#[derive(Serialize)]
pub struct SessionInitiativeEntry {
    target: EncodedUuid,
    value: i64
}

#[derive(Serialize)]
pub struct SessionInitiative {
    order: Vec<SessionInitiativeEntry>,
    round: u16,
    turn_idx: usize,
}

impl From<&Initiative> for SessionInitiative {
    fn from(value: &Initiative) -> Self {
        Self {
            round: value.round,
            order: value.order.iter().map(|(id, v)| {
                SessionInitiativeEntry { target: EncodedUuid(*id), value: *v }
            }).collect(),
            turn_idx: value.turn_idx,
        }
    }
}
