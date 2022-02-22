use std::io::Write;

use crate::cli::Cli;
use crate::results::*;
use crate::session::{Mode, Session, SysConfig};
use crate::token::TokenInfo;
use crate::CliError;
use chrono::{DateTime, Local};
use directories::BaseDirs;
use url::Url;

pub enum Command {
    Email,
    Event,
    Me,
}

#[derive(Debug)]
pub struct Program {
    pub session: Session,
    pub token: Option<TokenInfo>,
    pub config: Cli,
    login: String,
    pub grant_mode: Mode,
}

impl Program {
    pub async fn new(config: Cli) -> Result<Self, CliError> {
        if !(check_if_config_file_exists()) {
            create_config_toml()?;
            if let Ok(result) = check_config_toml() {
                if !result {
                    return Err(CliError::ConfigFileNotFound);
                }
            }
        }
        let name = config.user.clone();
        let sys_config = SysConfig::new()?;
        let session = Session::new(Mode::Credentials, &sys_config).await?;
        let program = Program {
            session,
            token: None,
            config,
            login: name.unwrap_or_else(|| sys_config.login()),
            grant_mode: Mode::Credentials,
        };
        Ok(program)
    }

    pub async fn call(&mut self, url: &str) -> Result<String, CliError> {
        let res = self.session.call(url).await?;
        Ok(res)
    }

    pub async fn run_program(&mut self, command: Command) -> Result<(), CliError> {
        let tmp = self.get_user_with_login().await?;
        let mut user = self.get_user_info_with_id(tmp.id).await?;
        match command {
            Command::Me => user.me(&self.config).await?,
            Command::Email => user.email(),
            Command::Event => self.event(&user).await?,
        }
        Ok(())
    }
}

// TODO:
// - Add a functions detail if needed. for --details option.
impl Program {
    // FIXME:
    // - can be used when code grant is implemented.
    #[allow(dead_code)]
    async fn get_me(&mut self) -> Result<me::Me, CliError> {
        let url = "https://api.intra.42.fr/v2/me";
        let url = Url::parse_with_params(url, &[("client_id", self.session.client_id())])?;
        let res = self.call(url.as_str()).await?;
        Ok(serde_json::from_str(res.as_str())?)
    }
    async fn get_user_with_login(&mut self) -> Result<user::UserElement, CliError> {
        let url = "https://api.intra.42.fr/v2/users";
        let url = Url::parse_with_params(
            url,
            &[
                ("client_id", self.session.client_id()),
                ("filter[login]", &self.login),
            ],
        )?;

        let res = self.call(url.as_str()).await?;
        let user: user::User = serde_json::from_str(res.as_str())?;
        if user.is_empty() {
            return Err(CliError::UserNotFound(self.login.clone()));
        }
        Ok(user[0].clone())
    }

    async fn get_user_info_with_id(&mut self, id: i64) -> Result<me::Me, CliError> {
        let url = format!("https://api.intra.42.fr/v2/users/{}", id);
        let url = Url::parse_with_params(&url, &[("client_id", self.session.client_id())])?;

        let res = self.call(url.as_str()).await?;
        let me: me::Me = serde_json::from_str(res.as_str())?;
        Ok(me)
    }

    async fn event(&mut self, user: &me::Me) -> Result<(), CliError> {
        let campus_id = user.campus[0].id;
        let url = format!("https://api.intra.42.fr/v2/campus/{}/events", campus_id);
        let url = Url::parse_with_params(&url, &[("client_id", self.session.client_id())])?;
        let res = self.call(url.as_str()).await?;
        let events: campus_event::CampusEvent = serde_json::from_str(res.as_str())?;

        let local = Local::now();
        for (_, event) in events.iter().rev().enumerate() {
            let begin = event.begin_at.parse::<DateTime<Local>>()?;
            let end = event.end_at.parse::<DateTime<Local>>()?;
            if end.signed_duration_since(local).num_seconds() > 0 {
                println!("🌈 🌈 🌈 {} 🌈 🌈 🌈\n", event.name);
                println!("⏰{:24}{}", "Begin at", begin);
                println!("⏰{:24}{}\n", "End at", end);
                if self.config.detail {
                    println!("{}\n", event.description);
                }
            }
        }
        Ok(())
    }
}

// TODO:
// move functions to member functions.
fn check_if_config_file_exists() -> bool {
    if let Some(dir) = BaseDirs::new() {
        return dir.config_dir().join("config.toml").exists();
    }
    false
}

fn create_config_toml() -> Result<(), CliError> {
    use std::fs::File;
    use std::io::stdin;

    println!("Browse to: https://profile.intra.42.fr/oauth/applications/new");
    println!("Create new Application");
    println!("Set redirect_url to \"http://localhost:8080\"");

    let dir = BaseDirs::new().ok_or(CliError::BaseDirsNewError)?;
    let path = dir.config_dir().join("config.toml");
    let mut file = File::create(path)?;
    let stdin = stdin();
    let mut line = String::new();

    println!("Enter intra login: ");
    stdin.read_line(&mut line)?;
    writeln!(&mut file, "login=\"{}\"", line.trim())?;
    line.clear();
    writeln!(&mut file, "[session]")?;
    line.clear();
    println!("Enter client id: ");
    stdin.read_line(&mut line)?;
    writeln!(&mut file, "client_id=\"{}\"", line.trim())?;
    line.clear();
    println!("Enter client secret: ");
    stdin.read_line(&mut line)?;
    writeln!(&mut file, "client_secret=\"{}\"", line.trim())?;
    Ok(())
}

fn check_config_toml() -> Result<bool, CliError> {
    use std::io::ErrorKind;

    if let Some(dir) = BaseDirs::new() {
        let path = dir.config_dir().join("config.toml");
        let tmp = std::fs::read_to_string(path);
        match tmp {
            Ok(content) => {
                let config: Session = toml::from_str(&content)?;
                if !(check_client(&config)) {
                    return Ok(false);
                }
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("config.toml not found");
                    return Ok(false);
                }
                ErrorKind::PermissionDenied => {
                    eprintln!("config.toml not readable");
                    return Ok(false);
                }
                _ => {
                    eprintln!("config.toml error.");
                    eprintln!("something went wrong.");
                }
            },
        }
    }
    Ok(true)
}

fn check_client(session: &Session) -> bool {
    let client_id = session.client_id();
    let client_secret = session.client_secret();
    if client_id.is_empty() || client_secret.is_empty() {
        return false;
    }
    if client_id.len() > 256 || client_secret.len() > 256 {
        return false;
    }
    true
}
