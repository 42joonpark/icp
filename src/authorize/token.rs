extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TokenInfo {
    #[serde(rename = "resource_owner_id")]
    resource_owner_id: Option<i64>,

    #[serde(rename = "scopes")]
    scopes: Option<Vec<String>>,

    #[serde(rename = "expires_in_seconds")]
    expires_in_seconds: Option<i64>,

    #[serde(rename = "application")]
    application: Option<Application>,

    #[serde(rename = "created_at")]
    created_at: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
    pub created_at: i64,
}

impl TokenInfo {
    pub fn new() -> TokenInfo {
        TokenInfo::default()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "uid")]
    uid: Option<String>,
}
