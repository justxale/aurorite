#[derive(Clone)]
pub struct EnvConfig {
    host: String,
    port: String,
    database_path: String,
}

impl EnvConfig {
    pub fn new() -> Self {
        match dotenvy::dotenv() {
            Ok(_) => Self {
                host: std::env::var("AURORITE_HOST").unwrap_or(String::from("0.0.0.0")),
                port: std::env::var("AURORITE_PORT").unwrap_or(String::from("11811")),
                database_path: std::env::var("AURORITE_DATABASE_PATH").unwrap_or(EnvConfig::default_db_path()),
            },
            Err(_) => EnvConfig::default()
        }
    }
    
    pub fn default_db_path() -> String {
        std::env::current_dir().unwrap().join("./aurorite.sqlite").into_os_string().into_string().unwrap()
    }

    pub fn host(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    pub fn db_path(&self) -> &String {
        &self.database_path
    }
}

impl Default for EnvConfig {
    fn default() -> Self {
        Self {
            host: String::from("0.0.0.0"),
            port: String::from("11811"),
            database_path: EnvConfig::default_db_path()
        }
    }
}