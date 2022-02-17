// - CampusEvent 는 campus event 정보를 담고 있습니다.
// - https://api.intra.42.fr/v2/campus/{}/events 이런식으로 호출할 때 사용됩니다.
// 
// - Program::event() 함수 호출 시 얻는 결과입니다.
// 
// - Paste Json as Code vscode extension을 사용했습니다.

extern crate serde_json;
use serde::{Deserialize, Serialize};

pub type CampusEvent = Vec<CampusEventElement>;

#[derive(Serialize, Deserialize)]
pub struct CampusEventElement {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "location")]
    pub location: String,

    #[serde(rename = "kind")]
    kind: String,

    #[serde(rename = "max_people")]
    pub max_people: Option<i64>,

    #[serde(rename = "nbr_subscribers")]
    nbr_subscribers: i64,

    #[serde(rename = "begin_at")]
    pub begin_at: String,

    #[serde(rename = "end_at")]
    pub end_at: String,

    #[serde(rename = "campus_ids")]
    campus_ids: Vec<i64>,

    #[serde(rename = "cursus_ids")]
    cursus_ids: Vec<i64>,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "prohibition_of_cancellation")]
    prohibition_of_cancellation: Option<i64>,

    #[serde(rename = "waitlist")]
    waitlist: Option<Waitlist>,

    #[serde(rename = "themes")]
    themes: Vec<Theme>,
}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Waitlist {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "waitlistable_id")]
    waitlistable_id: i64,

    #[serde(rename = "waitlistable_type")]
    waitlistable_type: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,
}
