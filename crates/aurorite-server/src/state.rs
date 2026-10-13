use crate::responses::AuroriteErrorResponse;
use crate::session::SessionManager;
use crate::traits::IntoJson;
use aurorite_dataflow::{build_connection, database::Db};
use aurorite_runtime::{Character, RuntimeCtx};
use axum::Json;
use axum::http::StatusCode;
use serde::Serialize;
use std::sync::{Arc, LazyLock};
use tokio::sync::broadcast::{channel, Sender};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

pub static SERVICE_EVENT_SENDER: LazyLock<Sender<ServiceEvent>> = LazyLock::new(|| {
    let (tx, _) = channel::<ServiceEvent>(16);
    tx
});

#[derive(Clone)]
pub struct AuroriteState {
    db: Db,
    pub manager: Arc<SessionManager>,
    pub cancellation_token: CancellationToken
}

impl AuroriteState {
    pub async fn new() -> Self {
        #[cfg(test)]
        let connection = build_connection::<true>().await;
        #[cfg(not(test))]
        let connection = build_connection::<false>().await;

        AuroriteState {
            db: connection.clone(),
            manager: Arc::new(SessionManager::new(connection)),
            cancellation_token: CancellationToken::new()
        }
    }

    pub fn db(&self) -> Db {
        self.db.clone()
    }

    pub fn session_and<F, C>(&self, session_id: Uuid, f: F) -> Result<C, (StatusCode, Json<AuroriteErrorResponse>)>
    where
        F: FnOnce(&mut RuntimeCtx) -> C
    {
        let session = self.manager.session(session_id).ok_or((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new("no session with this id").json(),
        ))?;
        let res = f(&mut session.ctx().lock());
        Ok(res)
    }

    pub fn session_character_and<F, C>(
        &self,
        session_id: Uuid,
        character_id: Uuid,
        f: F,
    ) -> Result<C, (StatusCode, Json<AuroriteErrorResponse>)>
    where
        F: FnOnce(&Character) -> C
    {
        self.session_and(session_id, |v| {
            v.character(character_id).map(f).ok_or((
                StatusCode::NOT_FOUND,
                AuroriteErrorResponse::new("no character with this id").json(),
            ))
        })?
    }

    pub async fn cleanup(self) {
        tracing::info!("cleaning up state");
        match Arc::try_unwrap(self.manager) {
            Err(_) => tracing::error!("error on cleanup, data will not be saved"),
            Ok(manager) => manager.cleanup().await,
        };
    }
}

#[derive(Serialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ServiceState {
    Stale,
    Processing
}

#[derive(Serialize, Copy, Clone)]
#[serde(tag = "ev", rename_all = "snake_case")]
pub enum ServiceEvent {
    RamUsage { current: usize },
    StateUpdate { new: ServiceState },
    ProcessUpdate { id: &'static str, current: u64, total: u64 },
    IndefiniteProcessUpdate { id: &'static str, is_finished: Option<bool> },
}

impl ServiceEvent {
    pub fn event_name(&self) -> &'static str {
        match self {
            ServiceEvent::RamUsage { .. } => "ram_usage",
            ServiceEvent::StateUpdate { .. } => "state_update",
            ServiceEvent::ProcessUpdate { .. } => "process_update",
            ServiceEvent::IndefiniteProcessUpdate { .. } => "indefinite_process_update"
        }
    }
}