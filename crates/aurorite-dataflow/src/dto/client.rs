use serde::{Deserialize, Serialize};
use crate::database::Client;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientDto {
    pub username: String,
    pub display_name: Option<String>,
}

impl From<&Client> for ClientDto {
    fn from(client: &Client) -> Self {
        Self {
            username: client.username.clone(),
            display_name: client.display_name.clone(),
        }
    }
}