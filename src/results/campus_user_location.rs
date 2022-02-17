// - CampusUserLocationElement 
// https://api.intra.42.fr/v2/campus_users/:campus_user_id/locations 이런식으로 호출할 때 사용됩니다. 
// campus 의 사용자 정보가 필요할 때 사용합니다.
//
// - Paste Json as Code vscode extension을 사용했습니다.

extern crate serde_json;
use serde::{Deserialize, Serialize};

pub type CampusUserLocation = Vec<CampusUserLocationElement>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampusUserLocationElement {
    #[serde(rename = "end_at")]
    pub end_at: Option<String>,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "begin_at")]
    pub begin_at: Option<String>,

    #[serde(rename = "primary")]
    primary: bool,

    #[serde(rename = "host")]
    pub host: Option<String>,

    #[serde(rename = "campus_id")]
    campus_id: i64,

    #[serde(rename = "user")]
    user: User,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "email")]
    email: String,

    #[serde(rename = "login")]
    login: String,

    #[serde(rename = "first_name")]
    first_name: String,

    #[serde(rename = "last_name")]
    last_name: String,

    #[serde(rename = "usual_full_name")]
    usual_full_name: String,

    #[serde(rename = "usual_first_name")]
    usual_first_name: Option<String>,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "phone")]
    phone: String,

    #[serde(rename = "displayname")]
    displayname: String,

    #[serde(rename = "image_url")]
    image_url: String,

    #[serde(rename = "new_image_url")]
    new_image_url: String,

    #[serde(rename = "staff?")]
    staff: bool,

    #[serde(rename = "correction_point")]
    correction_point: i64,

    #[serde(rename = "pool_month")]
    pool_month: String,

    #[serde(rename = "pool_year")]
    pool_year: String,

    #[serde(rename = "location")]
    location: Option<String>,

    #[serde(rename = "wallet")]
    wallet: i64,

    #[serde(rename = "anonymize_date")]
    anonymize_date: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "alumni")]
    alumni: bool,

    #[serde(rename = "is_launched?")]
    is_launched: bool,
}
