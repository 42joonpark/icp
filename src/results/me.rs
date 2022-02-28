// - Me 는
// - https://api.intra.42.fr/v2/me 또는
// - https://api.intra.42.fr/v2/users/{id} 이런식으로 호출할 때 사용됩니다.
//
// - Program::get_me() 또는
// - Program::get_user_with_login() 함수 호출 시 얻는 결과입니다.
//
// - Paste Json as Code vscode extension을 사용했습니다.

extern crate serde_json;
use super::super::cli::Cli;
use super::super::CliError;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

pub type User = Vec<UserElement>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Me {
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

    #[serde(rename = "groups")]
    groups: Vec<Option<serde_json::Value>>,

    #[serde(rename = "cursus_users")]
    cursus_users: Vec<CursusUser>,

    #[serde(rename = "projects_users")]
    projects_users: Vec<ProjectsUser>,

    #[serde(rename = "languages_users")]
    languages_users: Vec<LanguagesUser>,

    #[serde(rename = "achievements")]
    achievements: Vec<Achievement>,

    #[serde(rename = "titles")]
    titles: Vec<Title>,

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
    grade: Option<String>,

    #[serde(rename = "level")]
    level: f64,

    #[serde(rename = "skills")]
    skills: Vec<Skill>,

    #[serde(rename = "blackholed_at")]
    blackholed_at: Option<String>,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "begin_at")]
    begin_at: String,

    #[serde(rename = "end_at")]
    end_at: Option<String>,

    #[serde(rename = "cursus_id")]
    cursus_id: i64,

    #[serde(rename = "has_coalition")]
    has_coalition: bool,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "user")]
    user: UserElement,

    #[serde(rename = "cursus")]
    cursus: Cursus,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Cursus {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "created_at")]
    created_at: Option<String>,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "slug")]
    slug: String,

    #[serde(rename = "parent_id")]
    parent_id: Option<serde_json::Value>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Skill {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "level")]
    level: f64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserElement {
    #[serde(rename = "id")]
    pub id: i64,

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
    name: String,
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
    #[serde(rename = "creating_group")]
    CreatingGroup,

    #[serde(rename = "waiting_to_start")]
    WaitingToStart,

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

impl Me {
    pub async fn me(&self, config: &Cli) -> Result<(), CliError> {
        if config._me {
            self.print_pretty_name(config._detail, config._human);
            self.wallet(config._detail, config._human);
            self.correction_point(config._detail, config._human);
            self.cursus(config._detail, config._human);
            self.grade(config._detail, config._human);
            self.level(config._detail, config._human);
            self.blackhole(config._detail, config._human)?;
            self.location(config._detail, config._human);
        } else {
            if config._id {
                self.id(config._detail, config._human);
            }
            if config._login {
                self.login(config._detail, config._human);
            }
            if config._wallet {
                self.wallet(config._detail, config._human);
            }
            if config._point {
                self.correction_point(config._detail, config._human);
            }
            if config._grade {
                self.grade(config._detail, config._human);
            }
            if config._level {
                self.level(config._detail, config._human);
            }
            if config._blackhole {
                self.blackhole(config._detail, config._human)?;
            }
            if config._location {
                self.location(config._detail, config._human);
            }
        }
        Ok(())
    }
}

impl Me {
    // TODO:
    // - Add a functions detail if needed. for --details option.
    // TODO:
    // - add human readable description
    fn print_pretty_name(&self, _detail: bool, _human: bool) {
        let title = if self.titles.is_empty() {
            ""
        } else {
            self.titles[0].name.split(' ').next().unwrap_or("")
        };
        println!("{} | {} {}", self.displayname, title, self.login);
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    fn wallet(&self, _detail: bool, _human: bool) {
        if _human {
            println!("{:20}{}", "Wallet", self.wallet);
        } else {
            println!("{}", self.wallet);
        }
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    fn id(&self, _detail: bool, _human: bool) {
        if _human {
            println!("{:20}{}", "ID", self.id);
        } else {
            println!("{}", self.id);
        }
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    fn cursus(&self, _detail: bool, _human: bool) {
        if _human {
            println!(
                "{:20}{}",
                "Cursus",
                self.cursus_users[self.cursus_users.len() - 1].cursus.name
            );
        } else {
            println!(
                "{}",
                self.cursus_users[self.cursus_users.len() - 1].cursus.name
            );
        }
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    fn login(&self, _detail: bool, _human: bool) {
        if _human {
            println!("{:20}{}", "Login", self.login);
        } else {
            println!("{}", self.login);
        }
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    fn correction_point(&self, _detail: bool, _human: bool) {
        if _human {
            println!("{:20}{}", "Correction Point", self.correction_point);
        } else {
            println!("{}", self.correction_point);
        }
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    // TODO:
    // - fix _detail if _human is false.
    fn blackhole(&self, _detail: bool, _human: bool) -> Result<(), CliError> {
        if self.cursus_users[self.cursus_users.len() - 1]
            .blackholed_at
            .is_none()
        {
            return Ok(());
        }
        let local = Local::now();
        let local2 = self.cursus_users[self.cursus_users.len() - 1]
            .blackholed_at
            .as_ref()
            .unwrap_or(&"".to_string())
            .parse::<DateTime<Local>>()?;

        let remaining_days = local2.signed_duration_since(local).num_days();
        if _human {
            print!(
                "{:20}{}",
                "Blackhole",
                format!("{} day(s) remaining", remaining_days)
            );
            match remaining_days {
                1..=30 => println!(" 😱"),
                31..=60 => println!(" 😡"),
                _ => println!(" 🤪"),
            }
        } else {
            println!("{}", remaining_days);
        }
        if _detail {
            println!("{:19}{}\n", "⏰End at", local2);
        }
        Ok(())
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    fn grade(&self, _detail: bool, _human: bool) {
        if _human {
            println!(
                "{:20}{}",
                "Grade",
                self.cursus_users[self.cursus_users.len() - 1]
                    .grade
                    .as_ref()
                    .unwrap_or(&"".to_string())
            );
        } else {
            println!(
                "{}",
                self.cursus_users[self.cursus_users.len() - 1]
                    .grade
                    .as_ref()
                    .unwrap_or(&"".to_string())
            )
        }
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    fn location(&self, _detail: bool, _human: bool) {
        if let Some(loc) = &self.location {
            if _human {
                println!("{:20}{}", "Location", loc);
            } else {
                println!("{}", loc);
            }
        } else if _human {
            println!("{:20} Unknown", "Location");
        } else {
            println!("Unknown");
        }
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    fn level(&self, _detail: bool, _human: bool) {
        if _human {
            println!(
                "{:20}{}",
                "Level",
                self.cursus_users[self.cursus_users.len() - 1].level
            );
        } else {
            println!("{}", self.cursus_users[self.cursus_users.len() - 1].level);
        }
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    pub fn email(&self, _detail: bool, _human: bool) {
        if _human {
            println!("{:20}{}", "Email", self.email);
        } else {
            println!("{}", self.email);
        }
    }

    // TODO:
    // - Add a functions detail if needed. for --details option.
    // TODO:
    // - add human readable description
    pub fn projects(&self, _detail: bool, _human: bool) {
        if _human {
            println!("-- Projects --");
        }
        for project in &self.projects_users {
            match project.status {
                Status::InProgress | Status::WaitingForCorrection => {
                    println!("{}", project.project.name);
                }
                _ => {}
            }
        }
    }
}
