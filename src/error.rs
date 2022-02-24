use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ParseUrlError(#[from] url::ParseError),
    #[error(transparent)]
    ChoronoParseError(#[from] chrono::ParseError),

    #[error(transparent)]
    TomlDeError(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSerError(#[from] toml::ser::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    VarError(#[from] std::env::VarError),

    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    TokenError(#[from] TokenError),

    #[error("Error: {0}")]
    IcpError(String),

    #[error("Error: User not found.")]
    UserNotFound(String),
    #[error("Error: Configure file not found")]
    ConfigFileNotFound,
    #[error("Error: File does not exist")]
    BaseDirsNewError,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Error: Server Unauthorized")]
    UnauthResponse,
    #[error("Error: 403 Forbidden Access")]
    Forbidden,
    #[error("Error: 404 Page or resource is not found")]
    NotFound,
}

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum TokenError {
    #[error("Error: No access token found")]
    NoAccessToken,
    #[error("Error: Not valide token Error")]
    TokenNotValid,
    #[error("Error: Invalid refresh token")]
    InvalidRefreshToken,
}
