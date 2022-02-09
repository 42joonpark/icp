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
    VarError(#[from] std::env::VarError),
    #[error("Error: toml Error")]
    TomlError(#[from] toml::de::Error),
    #[error("Error: 401 Unauthorized Result")]
    Unauthorized,
    #[error("Error: Server Unauthorized")]
    ServerUnauthorized,
    #[error("Error: 403 Fobidden Access")]
    Fobidden,
    #[error("Error: 404 Page or resource is not found")]
    NotFound,
    #[error("Error: None found.")]
    NoneError,
    #[error("Error: Session Not exist")]
    SessionExistError,
    #[error("Error: Configure file not found")]
    ConfigFileNotFound,
}
