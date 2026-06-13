use aurorite_dataflow::database::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientToken {
    pub access_token: String,
    pub token_type: String,
}


