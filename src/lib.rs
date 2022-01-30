pub mod authorize;
pub mod command;
pub mod structs;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    DotenvError(#[from] dotenv::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    #[error("Error: 401 Unauthorized Result")]
    UnauthorizedResult,
    #[error("Error: 403 Fobidden Access")]
    Fobidden,
    #[error("Error: 404 Page or resource is not found")]
    NotFound,
    #[error("Error: None found.")]
    NoneError,
    #[error("Error: Session Not exist")]
    SessionExistError,
}
