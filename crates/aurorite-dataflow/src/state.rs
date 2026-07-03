use crate::database::Client;
use aurorite_util::{auth::hash_password, env};
use toasty::Db;

pub async fn build_connection<const IS_TEST: bool>() -> Db {
    let mut builder = Db::builder();
    builder.models(toasty::models!(crate::*));
    let mut connection = if IS_TEST {
        builder.connect("sqlite::memory:")
            .await
            .unwrap()
    } else {
        builder.connect(format!("sqlite:///{}?mode=rwc", env().database_path).as_str())
            .await
            .unwrap()
    };

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
