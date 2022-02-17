// - Me 는 
// - https://api.intra.42.fr/v2/me 또는
// - https://api.intra.42.fr/v2/users/{id} 이런식으로 호출할 때 사용됩니다.
// 
// - Program::get_me() 또는
// - Program::get_user_with_login() 함수 호출 시 얻는 결과입니다.
// 
// - Paste Json as Code vscode extension을 사용했습니다.

extern crate serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Me {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "login")]
    pub login: String,

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
    pub displayname: String,

    #[serde(rename = "image_url")]
    image_url: String,

    #[serde(rename = "staff?")]
    staff: bool,

    #[serde(rename = "correction_point")]
    pub correction_point: i64,

    #[serde(rename = "pool_month")]
    pool_month: String,

    #[serde(rename = "pool_year")]
    pool_year: String,

    #[serde(rename = "location")]
    pub location: Option<serde_json::Value>,

    #[serde(rename = "wallet")]
    pub wallet: i64,

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

    #[serde(rename = "groups")]
    groups: Vec<Option<serde_json::Value>>,

    #[serde(rename = "cursus_users")]
    pub cursus_users: Vec<CursusUser>,

    #[serde(rename = "projects_users")]
    projects_users: Vec<ProjectsUser>,

    #[serde(rename = "languages_users")]
    languages_users: Vec<LanguagesUser>,

    #[serde(rename = "achievements")]
    achievements: Vec<Achievement>,

    #[serde(rename = "titles")]
    pub titles: Vec<Title>,

    #[serde(rename = "titles_users")]
    titles_users: Vec<TitlesUser>,

    #[serde(rename = "partnerships")]
    partnerships: Vec<Option<serde_json::Value>>,

    #[serde(rename = "patroned")]
    patroned: Vec<Option<serde_json::Value>>,

    #[serde(rename = "patroning")]
    patroning: Vec<Option<serde_json::Value>>,

    #[serde(rename = "expertises_users")]
    expertises_users: Vec<ExpertisesUser>,

    #[serde(rename = "roles")]
    roles: Vec<Option<serde_json::Value>>,

    #[serde(rename = "campus")]
    pub campus: Vec<Campus>,

    #[serde(rename = "campus_users")]
    campus_users: Vec<CampusUser>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Achievement {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "tier")]
    tier: Tier,

    #[serde(rename = "kind")]
    kind: Kind,

    #[serde(rename = "visible")]
    visible: bool,

    #[serde(rename = "image")]
    image: Option<String>,

    #[serde(rename = "nbr_of_success")]
    nbr_of_success: Option<i64>,

    #[serde(rename = "users_url")]
    users_url: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Campus {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "time_zone")]
    time_zone: String,

    #[serde(rename = "language")]
    language: Language,

    #[serde(rename = "users_count")]
    users_count: i64,

    #[serde(rename = "vogsphere_id")]
    vogsphere_id: i64,

    #[serde(rename = "country")]
    country: String,

    #[serde(rename = "address")]
    address: String,

    #[serde(rename = "zip")]
    zip: String,

    #[serde(rename = "city")]
    city: String,

    #[serde(rename = "website")]
    website: String,

    #[serde(rename = "facebook")]
    facebook: String,

    #[serde(rename = "twitter")]
    twitter: String,

    #[serde(rename = "active")]
    active: bool,

    #[serde(rename = "email_extension")]
    email_extension: String,

    #[serde(rename = "default_hidden_phone")]
    default_hidden_phone: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Language {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "identifier")]
    identifier: String,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CampusUser {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "user_id")]
    user_id: i64,

    #[serde(rename = "campus_id")]
    campus_id: i64,

    #[serde(rename = "is_primary")]
    is_primary: bool,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CursusUser {
    #[serde(rename = "grade")]
    pub grade: Option<String>,

    #[serde(rename = "level")]
    pub level: f64,

    #[serde(rename = "skills")]
    pub skills: Vec<Skill>,

    #[serde(rename = "blackholed_at")]
    pub blackholed_at: Option<String>,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "begin_at")]
    pub begin_at: String,

    #[serde(rename = "end_at")]
    pub end_at: Option<String>,

    #[serde(rename = "cursus_id")]
    pub cursus_id: i64,

    #[serde(rename = "has_coalition")]
    pub has_coalition: bool,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String,

    #[serde(rename = "user")]
    pub user: User,

    #[serde(rename = "cursus")]
    pub cursus: Cursus,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Cursus {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "created_at")]
    pub created_at: Option<String>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "slug")]
    pub slug: String,

    #[serde(rename = "parent_id")]
    pub parent_id: Option<serde_json::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Skill {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "level")]
    pub level: f64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExpertisesUser {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "expertise_id")]
    expertise_id: i64,

    #[serde(rename = "interested")]
    interested: bool,

    #[serde(rename = "value")]
    value: i64,

    #[serde(rename = "contact_me")]
    contact_me: bool,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "user_id")]
    user_id: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LanguagesUser {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "language_id")]
    language_id: i64,

    #[serde(rename = "user_id")]
    user_id: i64,

    #[serde(rename = "position")]
    position: i64,

    #[serde(rename = "created_at")]
    created_at: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProjectsUser {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "occurrence")]
    occurrence: i64,

    #[serde(rename = "final_mark")]
    final_mark: Option<i64>,

    #[serde(rename = "status")]
    status: Status,

    #[serde(rename = "validated?")]
    validated: Option<bool>,

    #[serde(rename = "current_team_id")]
    current_team_id: Option<i64>,

    #[serde(rename = "project")]
    project: Cursus,

    #[serde(rename = "cursus_ids")]
    cursus_ids: Vec<i64>,

    #[serde(rename = "marked_at")]
    marked_at: Option<String>,

    #[serde(rename = "marked")]
    marked: bool,

    #[serde(rename = "retriable_at")]
    retriable_at: Option<String>,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Title {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TitlesUser {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "user_id")]
    user_id: i64,

    #[serde(rename = "title_id")]
    title_id: i64,

    #[serde(rename = "selected")]
    selected: bool,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "pedagogy")]
    Pedagogy,

    #[serde(rename = "project")]
    Project,

    #[serde(rename = "scolarity")]
    Scolarity,

    #[serde(rename = "social")]
    Social,
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Pedagogy
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Tier {
    #[serde(rename = "easy")]
    Easy,

    #[serde(rename = "hard")]
    Hard,

    #[serde(rename = "medium")]
    Medium,

    #[serde(rename = "none")]
    None,
}

impl Default for Tier {
    fn default() -> Self {
        Tier::None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "searching_a_group")]
    SearchingGroup,

    #[serde(rename = "finished")]
    Finished,

    #[serde(rename = "in_progress")]
    InProgress,

    #[serde(rename = "waiting_for_correction")]
    WaitingForCorrection,
}

impl Default for Status {
    fn default() -> Self {
        Status::WaitingForCorrection
    }
}
