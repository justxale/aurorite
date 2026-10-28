use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ClientAuth {
    #[serde(alias = "username")]
    pub login: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewClientData {
    pub nickname: String,
    pub display_name: String,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatedClientData {
    pub display_name: Option<String>,
    pub is_admin: Option<bool>,
    pub is_master: Option<bool>,
}
