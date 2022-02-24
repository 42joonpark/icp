use crate::error::{AuthError, CliError, TokenError};
use directories::BaseDirs;
use log::{self, debug, info};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

#[derive(Deserialize, Serialize, Debug)]
pub struct Client {
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl Client {
    pub async fn new() -> Result<Self, CliError> {
        let dir = BaseDirs::new().ok_or(CliError::BaseDirsNewError)?;
        let path = dir.config_dir().join("config.toml");
        let content = fs::read_to_string(path)
            .map_err(|_| CliError::IcpError("config.toml file does not exist!!".to_string()))?;
        let mut client: Client = toml::from_str(&content)?;
        if client.access_token().is_none() {
            client.generate_token().await?;
            client.to_file()?;
        } else {
            if !TokenInfo::check_token_valide(client.access_token()).await? {
                client.refresh().await?;
                client.to_file()?;
            }
        }
        Ok(client)
    }

    pub fn to_file(&self) -> Result<(), CliError> {
        let content = toml::to_string(&self)?;
        let dir = BaseDirs::new().ok_or(CliError::BaseDirsNewError)?;
        let path = dir.config_dir().join("config.toml");
        self.write_to_file(path.as_path(), content)?;
        Ok(())
    }

    pub fn client_id(&self) -> &str {
        self.client_id.as_str()
    }
    pub fn client_secret(&self) -> &str {
        self.client_secret.as_str()
    }
    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_ref().map(|s| s.as_str())
    }
    pub fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_ref().map(|s| s.as_str())
    }
    fn write_to_file(&self, filename: &Path, content: String) -> Result<(), CliError> {
        use std::io::Write;

        info!("write_to_file()");
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            // .append(true)
            .open(filename)?;
        writeln!(file, "{}", content).unwrap();
        Ok(())
    }
}

impl Client {
    async fn generate_token(&mut self) -> Result<(), CliError> {
        let client = BasicClient::new(
            ClientId::new(String::from(self.client_id())),
            Some(ClientSecret::new(String::from(self.client_secret()))),
            AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string())?,
            Some(TokenUrl::new(
                "https://api.intra.42.fr/oauth/token".to_string(),
            )?),
        )
        .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

        let (auth_url, _) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("public".to_string()))
            .url();
        println!("Browse to: {}", auth_url);

        self.local_server(client).await?;
        // self.access_token = Some(self.local_server(client).await?);
        Ok(())
    }

    async fn local_server(&mut self, client: BasicClient) -> Result<(), CliError> {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                let code;
                let state;
                {
                    let mut reader = BufReader::new(&mut stream);
                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).await?;
                    let redirect_url = match request_line.split_whitespace().nth(1) {
                        Some(url) => url,
                        None => {
                            return Err(CliError::IcpError("Failed to get redirect url.".into()))
                        }
                    };
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url))
                        .map_err(|_| CliError::IcpError("Failed to parse redirect url.".into()))?;

                    let code_pair = match url.query_pairs().find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    }) {
                        Some(code) => code,
                        None => return Err(CliError::IcpError("Failed to get code.".into())),
                    };

                    let (_, value) = code_pair;
                    code = AuthorizationCode::new(value.into_owned());

                    let state_pair = match url.query_pairs().find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    }) {
                        Some(state) => state,
                        None => return Err(CliError::IcpError("Failed to get state.".into())),
                    };

                    let (_, value) = state_pair;
                    state = CsrfToken::new(value.into_owned());
                }
                let message = "Go back to your terminal :)";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    message.len(),
                    message
                );
                stream
                    .write_all(response.as_bytes())
                    .await
                    .map_err(|_| CliError::IcpError("Failed to write response.".into()))?;

                debug!("42API returned the following code:\n{}\n", code.secret());
                debug!("42API returned the following state:\n{}\n", state.secret());

                let token_res = client
                    .exchange_code(code)
                    .request_async(async_http_client)
                    .await;
                let token = match token_res {
                    Err(_) => return Err(CliError::AuthError(AuthError::UnauthResponse)),
                    Ok(t) => t,
                };
                debug!("42API returned the following token:\n{:?}\n", token);

                let scopes = if let Some(scopes_vec) = token.scopes() {
                    scopes_vec
                        .iter()
                        .map(|comma_separated| comma_separated.split(','))
                        .flatten()
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                };
                self.access_token = Some(token.access_token().secret().to_owned());
                self.refresh_token = Some(
                    token
                        .refresh_token()
                        .ok_or(CliError::IcpError("refresh token not found.".to_string()))?
                        .secret()
                        .to_owned(),
                );
                debug!("Access Token: {:?}", self.access_token());
                debug!("Refresh Token: {:?}", self.refresh_token());
                debug!("42API returned the following scopes:\n{:?}\n", scopes);
                break;
            }
        }
        Ok(())
    }

    pub async fn refresh(&mut self) -> Result<(), CliError> {
        let client = BasicClient::new(
            ClientId::new(String::from(self.client_id())),
            Some(ClientSecret::new(String::from(self.client_secret()))),
            AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string())?,
            Some(TokenUrl::new(
                "https://api.intra.42.fr/oauth/token".to_string(),
            )?),
        )
        .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

        let token_res = client
            .exchange_refresh_token(&RefreshToken::new(
                self.refresh_token()
                    .ok_or(CliError::TokenError(TokenError::InvalidRefreshToken))?
                    .to_string(),
            ))
            .request_async(async_http_client)
            .await;
        let token = match token_res {
            Err(_) => return Err(CliError::AuthError(AuthError::UnauthResponse)),
            Ok(t) => t,
        };
        debug!("42API returned the following token:\n{:?}\n", token);

        let scopes = if let Some(scopes_vec) = token.scopes() {
            scopes_vec
                .iter()
                .map(|comma_separated| comma_separated.split(','))
                .flatten()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        self.access_token = Some(token.access_token().secret().to_owned());
        self.refresh_token = Some(token.refresh_token().unwrap().secret().to_owned());
        debug!("Refresh Token: {:?}", self.refresh_token());
        debug!("Access Token: {:?}", self.access_token());
        debug!("42API returned the following scopes:\n{:?}\n", scopes);
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct TokenInfo {
    #[serde(rename = "resource_owner_id")]
    pub resource_owner_id: Option<i64>,

    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,

    #[serde(rename = "expires_in_seconds")]
    pub expires_in_seconds: Option<i64>,

    #[serde(rename = "application")]
    _application: Option<Application>,

    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct Application {
    #[serde(rename = "uid")]
    _uid: Option<String>,
}

impl TokenInfo {
    pub async fn token_info(token: Option<&str>) -> Result<Option<TokenInfo>, CliError> {
        let url = "https://api.intra.42.fr/oauth/token/info";
        let url = Url::parse_with_params(url, &[("access_token", token.unwrap_or_default())])?;
        let resp = reqwest::get(url).await?;
        match resp.status() {
            reqwest::StatusCode::OK => {
                let token_info: TokenInfo = resp.json().await?;
                Ok(Some(token_info))
            }
            reqwest::StatusCode::UNAUTHORIZED => Ok(None),
            _ => Err(CliError::IcpError(
                "Something unexpected happened while getting token info.".to_string(),
            )),
        }
    }

    #[allow(dead_code)]
    pub async fn check_token_valide(token: Option<&str>) -> Result<bool, CliError> {
        let token_info = TokenInfo::token_info(token).await?;
        Ok(token_info.is_some())
    }
}
