use crate::responses::AuroriteErrorResponse;
use crate::session::SessionManager;
use crate::traits::IntoJson;
use aurorite_dataflow::{build_connection, database::Db};
use aurorite_runtime::Character;
use axum::http::StatusCode;
use axum::Json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuroriteState {
    db: Db,
    pub manager: Arc<SessionManager>,
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
        }
    }

    pub fn db(&self) -> Db {
        self.db.clone()
    }

    pub async fn session_character_and<F, C>(
        &self,
        session_id: Uuid,
        character_id: Uuid,
        f: F,
    ) -> Result<C, (StatusCode, Json<AuroriteErrorResponse>)>
    where
        F: FnOnce(&Character) -> C,
    {
        let session = self.manager.session(session_id).ok_or((
            StatusCode::NOT_FOUND,
            AuroriteErrorResponse::new("no session with this id").json(),
        ))?;
        session
            .ctx()
            .lock()
            .await
            .character(character_id)
            .map(f)
            .ok_or((
                StatusCode::NOT_FOUND,
                AuroriteErrorResponse::new("no character with this id").json(),
            ))
    }

    pub async fn cleanup(self) {
        tracing::info!("cleaning up state");
        match Arc::try_unwrap(self.manager) {
            Err(_) => tracing::error!("error on cleanup, data will not be saved"),
            Ok(manager) => manager.cleanup().await,
        };
    }
}
