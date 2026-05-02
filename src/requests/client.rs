use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ClientAuth {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct NewClientData {
    pub nickname: String,
    pub display_name: String,
    pub password: Option<String>,
}