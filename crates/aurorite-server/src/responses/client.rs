use crate::database::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientToken {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientInfo {
    pub username: String,
    pub display_name: Option<String>,
}

impl From<&Client> for ClientInfo {
    fn from(client: &Client) -> Self {
        Self {
            username: client.username.clone(),
            display_name: client.display_name.clone(),
        }
    }
}
