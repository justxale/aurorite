use toasty::Db;
use aurorite_util::{env, auth::hash_password};
use crate::database::Client;


pub async fn build_connection() -> Db {
    #[cfg(not(test))]
    let mut connection = Db::builder()
        .models(toasty::models!(crate::*))
        .connect(format!("sqlite:///{}?mode=rwc", env().database_path).as_str())
        .await
        .unwrap();

    #[cfg(test)]
    let mut connection = Db::builder()
        .models(toasty::models!(crate::*))
        .connect("sqlite::memory:")
        .await
        .unwrap();

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