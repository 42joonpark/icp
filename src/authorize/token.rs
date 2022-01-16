extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Serialize, Deserialize)]
pub struct TokenInfo {
    #[serde(rename = "resource_owner_id")]
    resource_owner_id: i64,

    #[serde(rename = "scopes")]
    scopes: Vec<String>,

    #[serde(rename = "expires_in_seconds")]
    expires_in_seconds: i64,

    #[serde(rename = "application")]
    application: Application,

    #[serde(rename = "created_at")]
    created_at: i64,
}

impl TokenInfo {
    pub fn new() -> TokenInfo {
        TokenInfo {
            resource_owner_id: -1,
            scopes: Vec::new(),
            expires_in_seconds: 0,
            application: Application::new(),
            created_at: 0,
        }
    }
}

impl fmt::Debug for TokenInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[\n\tresource_owner_id: {:?}\n\tscopes: {:?}\n\
                \texpires_in_secods: {:?}\n\
                \tapplication: {:?}\n\tcreated_at: {:?}\n]",
            self.resource_owner_id,
            self.scopes,
            self.expires_in_seconds,
            self.application,
            self.created_at
        )
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "uid")]
    uid: String,
}

impl Application {
    fn new() -> Application {
        Application {
            uid: String::from(""),
        }
    }
}

impl fmt::Debug for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.uid)
    }
}

