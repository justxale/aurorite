use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ClientToken {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClientInfo {
    pub nickname: String,
    pub display_name: Option<String>,
}