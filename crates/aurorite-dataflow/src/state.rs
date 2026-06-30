use toasty::Db;
use aurorite_util::{env, auth::hash_password};
use crate::database::Client;

#[cfg(debug_assertions)]
async fn build() -> Db {
    Db::builder()
        .models(toasty::models!(crate::*))
        .connect("sqlite::memory:")
        .await
        .unwrap()
}

#[cfg(not(debug_assertions))]
async fn build() -> Db {
    Db::builder()
        .models(toasty::models!(crate::*))
        .connect(format!("sqlite:///{}?mode=rwc", env().database_path).as_str())
        .await
        .unwrap()
}

pub async fn build_connection() -> Db {
    let mut connection = build().await;

    let _ = connection.push_schema().await;
    if let Ok(None) = Client::filter(Client::fields().is_admin().eq(true))
        .first()
        .exec(&mut connection)
        .await
    {
        Client::create()
            .username(env().admin.clone())
            .pwd(hash_password(&env().password).unwrap())
            .is_admin(true)
            .exec(&mut connection)
            .await
            .unwrap();
    }
    connection
}