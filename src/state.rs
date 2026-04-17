use std::sync::Arc;
use axum::extract::ws::Message;
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use vismut_core::VismutExecutionEnvironment;
use crate::config::EnvConfig;

#[derive(Clone)]
pub struct AuroriteState {
    connection: DatabaseConnection,
    env: Arc<EnvConfig>,
    executor: Arc<VismutExecutionEnvironment>,
    sender: broadcast::Sender<Message>
}


impl AuroriteState {
    pub async fn new() -> Self {
        let env = Arc::new(EnvConfig::new());
        let (tx, _) = broadcast::channel(100);
        let mut executor = VismutExecutionEnvironment::default();
        executor.get_schema_mut();
        let connection = Database::connect(
            format!("sqlite://{}?mode=rwc", env.db_path())
        ).await.unwrap();
        connection
            .get_schema_registry("aurorite::database")
            .sync(&connection)
            .await
            .unwrap();
        AuroriteState { connection, env, executor: Arc::new(executor), sender: tx }
    }
    
    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
    
    pub fn get_env(&self) -> &EnvConfig { &self.env }
    pub fn get_executor(&self) -> &VismutExecutionEnvironment {
        &self.executor
    }
    pub fn sender(&self) -> &Sender<Message> {
        &self.sender
    }
}
