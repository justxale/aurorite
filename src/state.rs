use crate::config::EnvConfig;
use axum::extract::ws::Message;
use std::sync::Arc;
use toasty::Db;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use vismut_core::VismutExecutionEnvironment;

#[derive(Clone, Debug)]
pub struct AuroriteState {
    db: Db,
    env: Arc<EnvConfig>,
    executor: Arc<VismutExecutionEnvironment>,
    sender: broadcast::Sender<Message>,
}

impl AuroriteState {
    pub async fn new() -> Self {
        let env = Arc::new(EnvConfig::new());
        let (tx, _) = broadcast::channel(100);
        let mut executor = VismutExecutionEnvironment::default();
        executor.get_schema_mut();

        let connection = Db::builder()
            .models(toasty::models!(crate::*))
            .connect(format!("sqlite:///{}?mode=rwc", env.db_path()).as_str())
            .await
            .unwrap();

        let _ = connection.push_schema().await;
        AuroriteState {
            db: connection,
            env,
            executor: Arc::new(executor),
            sender: tx,
        }
    }

    pub fn db(&self) -> Db {
        self.db.clone()
    }

    pub fn env(&self) -> &EnvConfig {
        &self.env
    }
    pub fn executor(&self) -> &VismutExecutionEnvironment {
        &self.executor
    }
    pub fn sender(&self) -> &Sender<Message> {
        &self.sender
    }
}
