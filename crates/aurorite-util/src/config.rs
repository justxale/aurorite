use config::{Config, ConfigError};
use serde::Deserialize;
use std::net::Ipv4Addr;
use std::sync::OnceLock;

#[cfg_attr(test, allow(dead_code))] // omit warn on unused database_path because of in-memory sqlite
#[derive(Deserialize)]
pub struct EnvConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub database_path: String,
    pub admin: String,
    pub password: String,
    pub secret: String,
    pub log: String,
    pub allow_cors: bool,
}

static CONFIG: OnceLock<EnvConfig> = OnceLock::new();
pub fn env() -> &'static EnvConfig {
    CONFIG.get_or_init(|| {
        let config = Config::builder()
            .set_default("host", "0.0.0.0")
            .unwrap()
            .set_default("port", 11811)
            .unwrap()
            .set_default("admin", "aurorite")
            .unwrap()
            .set_default("password", "aurorite")
            .unwrap()
            .set_default(
                "database_path",
                std::env::current_dir()
                    .unwrap()
                    .join("aurorite.sqlite")
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            )
            .unwrap()
            .set_default("log", "vismut_core=INFO,aurorite=INFO")
            .unwrap()
            .set_default("allow_cors", false)
            .unwrap()
            .add_source(
                config::Environment::with_prefix("AURORITE")
                    .ignore_empty(true)
            )
            .build()
            .unwrap();
        match config.try_deserialize::<EnvConfig>() {
            Ok(config) => config,
            Err(ConfigError::Message(msg)) => panic!("{msg}"),
            Err(ConfigError::NotFound(field)) => panic!("{field} must be set"),
            Err(ConfigError::At {
                    origin: _,
                    key,
                    error,
                }) => panic!("invalid value of {} field: {error}", key.unwrap()),
            Err(_) => panic!("unspecified error in configuration"),
        }
    })
}
