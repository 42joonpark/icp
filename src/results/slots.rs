extern crate serde_json;

use serde::{Deserialize, Serialize};

pub type Slots = Vec<Slot>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "begin_at")]
    begin_at: String,

    #[serde(rename = "end_at")]
    end_at: String,

    #[serde(rename = "scale_team")]
    scale_team: Option<ScaleTeam>,

    #[serde(rename = "user")]
    user: Option<UserElement>,
}

impl Slot {
	pub fn begin_at(&self) -> String {
		self.begin_at.clone()
	}
	pub fn end_at(&self) -> String {
		self.end_at.clone()
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleTeam {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "scale_id")]
    scale_id: i64,

    #[serde(rename = "comment")]
    comment: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "feedback")]
    feedback: String,

    #[serde(rename = "final_mark")]
    final_mark: i64,

    #[serde(rename = "flag")]
    flag: Flag,

    #[serde(rename = "begin_at")]
    begin_at: String,

    #[serde(rename = "correcteds")]
    correcteds: Vec<Correct>,

    #[serde(rename = "corrector")]
    corrector: Correct,

    #[serde(rename = "truant")]
    truant: Truant,

    #[serde(rename = "filled_at")]
    filled_at: String,

    #[serde(rename = "questions_with_answers")]
    questions_with_answers: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Correct {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "login")]
    login: String,

    #[serde(rename = "url")]
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Flag {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "positive")]
    positive: bool,

    #[serde(rename = "icon")]
    icon: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Truant {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserElement {
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
    usual_first_name: Option<serde_json::Value>,

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
    location: Option<serde_json::Value>,

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
