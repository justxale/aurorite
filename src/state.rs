use crate::config::env;
use axum::extract::ws::Message;
use std::sync::Arc;
use toasty::Db;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;
use vismut_core::VismutExecutionEnvironment;
use crate::database::Client;
use crate::utils::auth::hash_password;

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

        let connection = Self::build_connection().await;

        let _ = connection.push_schema().await;
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

    async fn build_connection() -> Db {
        let mut connection = Db::builder()
            .models(toasty::models!(crate::*))
            .connect(format!("sqlite:///{}?mode=rwc", env().database_path).as_str())
            .await
            .unwrap();
        if let Ok(None) = Client::filter(Client::fields().is_admin().eq(true)).first().exec(&mut connection).await {
            toasty::create!( Client {
                nickname: env().admin.clone(),
                pwd: hash_password(&env().password).unwrap()
            } ).exec(&mut connection).await.unwrap();
        }
        connection
    }
}
