extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Serialize, Deserialize)]
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

impl TokenInfo {
    pub fn new() -> TokenInfo {
        TokenInfo::default()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "uid")]
    uid: Option<String>,
}
