use crate::error::CliError;
use crate::error::{AuthError, TokenError};
use crate::token;
use directories::BaseDirs;
use log::{self, debug, warn};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

// Authorization grant type.
#[derive(Debug, Deserialize)]
pub enum Mode {
    Code,
    Credentials,
}

#[derive(Deserialize)]
pub struct SysConfig {
    session: Session,
    login: String,
}

impl SysConfig {
    // Creates a new instance of a `SysConfig`.
    // # Example
    // ```
    // let sys_config = SysConfig::new_with_path(PathBuf::from("/Users/someuser/Library/ApplicationSupport/config.toml"))?;
    // ```
    pub fn new_with_path(path: PathBuf) -> Result<Self, CliError> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
    pub fn new() -> Result<Self, CliError> {
        let dir = BaseDirs::new().ok_or(CliError::BaseDirsNewError)?;
        let path = dir.config_dir().join("config.toml");
        SysConfig::new_with_path(path)
    }
    pub fn login(&self) -> String {
        self.login.clone()
    }
    pub fn session(&self) -> Session {
        self.session.clone()
    }
}

// Build a session information.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Session {
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
}

impl Session {
    // Creates a new instance of a `Session`.
    //
    // It is required to have a `config.toml` file in the user's conig directory.\
    // Default authorization method is credentials grant. \
    //
    // # Example
    // ```
    // use icp::Session;
    // use icp::CliError;
    //
    // let session: Session = Session::new(None)?;
    // let session: Session = Session::new(Some(Mode::Code))?;
    // let session: Session = Session::new(Some(Mode::Credentials))?;
    // ```
    pub async fn new(m: Mode, config: &SysConfig) -> Result<Self, CliError> {
        let mut session: Session = config.session();
        match m {
            Mode::Code => {
                session.generate_token().await?;
            }
            Mode::Credentials => {
                session.generate_token_credentials().await?;
            }
        }
        Ok(session)
    }
}

impl Session {
    // Send GET request to given uri. \
    // Need valid access token.
    //
    // # Example
    // ```
    // use icp::Session;
    // use icp::CliError;
    // use icp::Mode;
    //
    // let session: Session = Session::new(Mode::Credentials)?;
    // let result = session.call("https://api.intra.42.fr/v2/users")?;
    // ```
    pub async fn call(&self, uri: &str) -> Result<String, CliError> {
        if self.access_token.is_none() {
            warn!("No access_token found");
            return Err(CliError::TokenError(TokenError::NoAccessToken));
        }
        let ac_token = self.access_token.clone().unwrap_or_default();
        let client = reqwest::Client::new();
        let params = [("client_id", self.client_id())];
        debug!("{}", ac_token);
        let response = client
            .get(uri.to_string())
            .header(AUTHORIZATION, format!("Bearer {}", ac_token))
            .form(&params)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                debug!("icp::Session::call(): reqwest OK");
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                warn!("icp::Session::call(): unauthorized");
                // return Err(CliError::UnauthorizedResponse);
                return Err(CliError::AuthError(AuthError::UnauthResponse));
            }
            reqwest::StatusCode::FORBIDDEN => {
                warn!("icp::Session::call(): 402 FORBIDDEN ACCESS");
                return Err(CliError::AuthError(AuthError::Forbidden));
            }
            reqwest::StatusCode::NOT_FOUND => {
                warn!("icp::Session::call(): 404 NOT FOUND");
                return Err(CliError::AuthError(AuthError::NotFound));
            }
            _ => {
                panic!("uh oh! something unexpected happened");
            }
        }
        let tmp = response.text().await?;
        Ok(tmp)
    }

    // Generate new access_token and asign it to the session
    pub async fn generate_token_credentials(&mut self) -> Result<(), CliError> {
        self.access_token = Some(token::generate_token_credentials(self.clone()).await?);
        Ok(())
    }
    pub async fn generate_token(&mut self) -> Result<(), CliError> {
        self.access_token = Some(token::generate_token(self.clone()).await?);
        Ok(())
    }
    // Get the `client_id` of the session
    pub fn client_id(&self) -> &str {
        self.client_id.as_str()
    }
    // Get the `client_secret` of the session
    pub fn client_secret(&self) -> &str {
        self.client_secret.as_str()
    }
    #[allow(dead_code)]
    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }
}
