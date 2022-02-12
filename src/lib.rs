pub mod results;
pub mod token;

use directories::BaseDirs;
use log::{self, debug, warn};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::fs;
use thiserror::Error;

/// Error type
#[derive(Error, Debug)]
pub enum SessionError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseUrlError(#[from] url::ParseError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error("Error: toml Error")]
    TomlError(#[from] toml::de::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error(transparent)]
    ChoronoParseError(#[from] chrono::ParseError),
    #[error("Error: No access token found")]
    TokenNotFound,
    #[error("Error: Not valide token Error")]
    TokenNotValid,
    #[error("Error: NoneError")]
    NoneError,
    #[error("Error: Server Unauthorized")]
    UnauthorizedServerError,
    #[error("Error: 403 Fobidden Access")]
    Fobidden,
    #[error("Error: 404 Page or resource is not found")]
    NotFound,
    #[error("Error: Configure file not found")]
    ConfigFileNotFound,
}

/// Authorization grant type.
pub enum Mode {
    Code,
    Credentials,
}

/// Build a session information.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Session {
    client_id: String,
    client_secret: String,
    login: String,
    access_token: Option<String>,
}

impl Session {
    /// Creates a new instance of a `Session`.
    ///
    /// It is required to have a `config.toml` file in the path directory.
    ///
    /// # Example
    /// ```
    /// let session: Session = Session::new()?;
    /// ```
    pub async fn new_with_path(path: &str, m: Option<Mode>) -> Result<Self, SessionError> {
        let content = fs::read_to_string(path)?;
        let mut session: Session = toml::from_str(&content)?;
        if let Some(mode) = m {
            match mode {
                Mode::Code => {
                    session.generate_token().await?;
                }
                Mode::Credentials => {
                    session.generate_token_credentials().await?;
                }
            }
        } else {
            session.generate_token_credentials().await?;
        }
        Ok(session)
    }

    /*
    /// Creates a new instance of a `Session`.
    ///
    /// It is required to have a `config.toml` file in the user's conig directory.\
    /// Default authorization method is credentials grant. \
    ///
    /// # Example
    /// ```
    /// use ftapi::Session;
    /// use ftapi::SessionError;
    ///
    /// let session: Session = Session::new(None)?;
    /// let session: Session = Session::new(Some(Mode::Code))?;
    /// let session: Session = Session::new(Some(Mode::Credentials))?;
    /// ```
     */
    pub async fn new(m: Option<Mode>) -> Result<Self, SessionError> {
        if let Some(dir) = BaseDirs::new() {
            let path = dir.config_dir().join("config.toml");
            let content = fs::read_to_string(path)?;
            let mut session: Session = toml::from_str(&content)?;
            if let Some(mode) = m {
                match mode {
                    Mode::Code => {
                        session.generate_token().await?;
                    }
                    Mode::Credentials => {
                        session.generate_token_credentials().await?;
                    }
                }
            } else {
                session.generate_token_credentials().await?;
            }
            Ok(session)
        } else {
            Err(SessionError::NoneError)
        }
    }
}

impl Session {
    /// Send GET request to given uri. \
    /// Need valid access token.
    ///
    /// # Example
    /// ```
    /// use ftapi::Session;
    /// use ftapi::SessionError;
    /// use ftapi::Mode;
    ///
    /// let session: Session = Session::new(Mode(None))?;
    /// let result = session.call("https://api.intra.42.fr/v2/users")?;
    /// ```
    pub async fn call(&self, uri: &str) -> Result<String, SessionError> {
        if self.access_token.is_none() {
            warn!("No access_token found");
            return Err(SessionError::TokenNotFound);
        }
        let ac_token = self.access_token.clone().unwrap_or_default();
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", self.get_client_id()),
        ];
        let response = client
            .get(uri.to_string())
            .header(AUTHORIZATION, format!("Bearer {}", ac_token))
            .form(&params)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                debug!("call(): reqwest OK");
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                warn!("call(): unauthorized");
                return Err(SessionError::UnauthorizedServerError);
            }
            reqwest::StatusCode::FORBIDDEN => {
                warn!("call(): 402 FORBIDDEN ACCESS");
                return Err(SessionError::Fobidden);
            }
            reqwest::StatusCode::NOT_FOUND => {
                warn!("404 NOT FOUND");
                return Err(SessionError::NotFound);
            }
            _ => {
                panic!("uh oh! something unexpected happened");
            }
        }
        let tmp = response.text().await?;
        Ok(tmp)
    }

    /// Generate new access_token and asign it to the session
    pub async fn generate_token_credentials(&mut self) -> Result<(), SessionError> {
        self.access_token = Some(token::generate_token_credentials(self.clone()).await?);
        Ok(())
    }
    pub async fn generate_token(&mut self) -> Result<(), SessionError> {
        self.access_token = Some(token::generate_token(self.clone()).await?);
        Ok(())
    }
    /// Get the `login` of the session
    pub fn get_login(&self) -> &str {
        self.login.as_str()
    }
    /// Get the `client_id` of the session
    pub fn get_client_id(&self) -> &str {
        self.client_id.as_str()
    }
    /// Get the `client_secret` of the session
    pub fn get_client_secret(&self) -> &str {
        self.client_secret.as_str()
    }
    /// Get the `access_token` of the session
    pub fn get_access_token(&self) -> Option<String> {
        self.access_token.clone()
    }
    /// Set the `access_token` of the session with a new value
    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }
}
