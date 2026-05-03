use std::sync::OnceLock;
use config::Config;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct EnvConfig {
    pub host: String,
    pub port: String,
    pub database_path: String,
    pub admin: String,
    pub password: String,
}

static CONFIG: OnceLock<EnvConfig> = OnceLock::new();
pub fn env() -> &'static EnvConfig {
    CONFIG.get_or_init(|| {
        Config::builder()
            .add_source(config::Environment::with_prefix("AURORITE")
                .try_parsing(true)
                .separator("_")
                .ignore_empty(true)
            )
            .set_default("host", "0.0.0.0").unwrap()
            .set_default("port", "11811").unwrap()
            .set_default("admin", "aurorite").unwrap()
            .set_default("password", "aurorite").unwrap()
            .set_default(
                "database_path",
                std::env::current_dir()
                    .unwrap()
                    .join("aurorite.sqlite")
                    .into_os_string()
                    .into_string()
                    .unwrap()
            )
            .unwrap().build().unwrap()
            .try_deserialize::<EnvConfig>().unwrap()
    })
}