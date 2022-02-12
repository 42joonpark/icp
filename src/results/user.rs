extern crate serde_json;
use serde::{Deserialize, Serialize};

pub type User = Vec<UserElement>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserElement {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "login")]
    pub login: String,

    #[serde(rename = "first_name")]
    pub first_name: String,

    #[serde(rename = "last_name")]
    pub last_name: String,

    #[serde(rename = "usual_full_name")]
    pub usual_full_name: String,

    #[serde(rename = "usual_first_name")]
    pub usual_first_name: Option<serde_json::Value>,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "phone")]
    pub phone: String,

    #[serde(rename = "displayname")]
    pub displayname: String,

    #[serde(rename = "image_url")]
    pub image_url: String,

    #[serde(rename = "staff?")]
    pub staff: bool,

    #[serde(rename = "correction_point")]
    pub correction_point: i64,

    #[serde(rename = "pool_month")]
    pub pool_month: String,

    #[serde(rename = "pool_year")]
    pub pool_year: String,

    #[serde(rename = "location")]
    pub location: Option<serde_json::Value>,

    #[serde(rename = "wallet")]
    pub wallet: i64,

    #[serde(rename = "anonymize_date")]
    pub anonymize_date: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "alumni")]
    pub alumni: bool,

    #[serde(rename = "is_launched?")]
    pub is_launched: bool,
}
