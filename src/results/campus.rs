// TODO:
// 왜 이렇게 만들었는지, 어떤 툴 썼는지 적어주세요. 

extern crate serde_json;
use serde::{Deserialize, Serialize};

pub type Campus = Vec<CampusElement>;

#[derive(Debug, Serialize, Deserialize)]
pub struct CampusElement {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "time_zone")]
    pub time_zone: String,

    #[serde(rename = "language")]
    pub language: Language,

    #[serde(rename = "users_count")]
    pub users_count: i64,

    #[serde(rename = "vogsphere_id")]
    vogsphere_id: Option<i64>,

    #[serde(rename = "country")]
    pub country: String,

    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "zip")]
    zip: String,

    #[serde(rename = "city")]
    pub city: String,

    #[serde(rename = "website")]
    pub website: String,

    #[serde(rename = "facebook")]
    pub facebook: String,

    #[serde(rename = "twitter")]
    pub twitter: String,

    #[serde(rename = "active")]
    pub active: bool,

    #[serde(rename = "email_extension")]
    email_extension: Option<String>,

    #[serde(rename = "default_hidden_phone")]
    default_hidden_phone: bool,

    #[serde(rename = "endpoint")]
    endpoint: Option<Endpoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "identifier")]
    pub identifier: String,
}
