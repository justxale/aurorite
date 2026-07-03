use aurorite_dataflow::{database::Db, build_connection};
use std::sync::Arc;
use crate::session::SessionManager;

#[derive(Clone, Debug)]
pub struct AuroriteState {
    db: Db,
    pub manager: Arc<SessionManager>
}

impl AuroriteState {
    pub async fn new() -> Self {
        #[cfg(test)]
        let connection = build_connection::<true>().await;
        #[cfg(not(test))]
        let connection = build_connection::<false>().await;
        AuroriteState {
            db: connection,
            manager: Arc::new(SessionManager::new())
        }
    }

    pub fn db(&self) -> Db {
        self.db.clone()
    }

    pub async fn cleanup(self) {
        tracing::info!("cleaning up state");
        let manager = Arc::into_inner(self.manager).expect("error on cleanup, data will not be saved");
        manager.cleanup(self.db).await;
    }
}
