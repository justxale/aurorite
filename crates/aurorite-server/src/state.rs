use aurorite_dataflow::{database::Db, build_connection};
use axum::extract::ws::Message;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use vismut_core::VismutExecutionEnvironment;

#[derive(Clone, Debug)]
pub struct AuroriteState {
    db: Db,
    executor: Arc<VismutExecutionEnvironment>,
    sender: broadcast::Sender<Message>,
}

impl AuroriteState {
    pub async fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        let mut executor = VismutExecutionEnvironment::default();
        executor.get_schema_mut();

        let connection = build_connection().await;
        AuroriteState {
            db: connection,
            executor: Arc::new(executor),
            sender: tx,
        }
    }

    pub fn db(&self) -> Db {
        self.db.clone()
    }
    pub fn executor(&self) -> &VismutExecutionEnvironment {
        &self.executor
    }
    pub fn sender(&self) -> &Sender<Message> {
        &self.sender
    }
}
