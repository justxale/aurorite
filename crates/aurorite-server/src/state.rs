use aurorite_dataflow::{database::Db, build_connection};
use std::sync::Arc;
use vismut_core::VismutExecutionEnvironment;
use crate::session::SessionManager;

#[derive(Clone, Debug)]
pub struct AuroriteState {
    db: Db,
    executor: Arc<VismutExecutionEnvironment>,
    pub manager: Arc<SessionManager>
}

impl AuroriteState {
    pub async fn new() -> Self {
        let mut executor = VismutExecutionEnvironment::default();
        executor.get_schema_mut();

        let connection = build_connection().await;
        AuroriteState {
            db: connection,
            executor: Arc::new(executor),
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
