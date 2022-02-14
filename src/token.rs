use crate::Session;
use crate::SessionError;
use log::{self, debug, info};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenInfo {
    #[serde(rename = "resource_owner_id")]
    pub resource_owner_id: Option<i64>,

    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,

    #[serde(rename = "expires_in_seconds")]
    pub expires_in_seconds: Option<i64>,

    #[serde(rename = "application")]
    application: Option<Application>,

    #[serde(rename = "created_at")]
    pub created_at: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Application {
    #[serde(rename = "uid")]
    uid: Option<String>,
}

// Try to get access token info.
//
// # Example
// ```
// let token_info: TokenInfo = token_info("Some Token");
// ```
pub async fn token_info(token: Option<String>) -> Result<TokenInfo, SessionError> {
    let url = "https://api.intra.42.fr/oauth/token/info";
    let url = Url::parse_with_params(url, &[("access_token", token.unwrap_or_default())])?;
    let resp = reqwest::get(url).await?;
    let token_info = resp.json::<TokenInfo>().await?;
    Ok(token_info)
}

// Check if the token is valid.
//
// # Example
// ```
// let res = check_token_valide(Some("Some Token".to_string())).await?;
// ```
pub async fn check_token_valide(token: Option<String>) -> Result<bool, SessionError> {
    let token_info = token_info(token).await?;
    if token_info.expires_in_seconds.is_none() {
        return Ok(false);
    }
    Ok(true)
}

#[derive(Deserialize, Debug)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
    pub created_at: i64,
}

// Generate credentials grant token.
//
// # Example
//
// ```
// let session = Session::new(Some(Mode::Credentials)).await?;
// let access_token = generate_token_credentials(session.clone()).await?;
// ```
pub async fn generate_token_credentials(session: Session) -> Result<String, SessionError> {
    info!("token::generate_token_credentials(): Begin");
    let client_id = session.client_id.to_owned();
    let client_secret = session.client_secret.to_owned();
    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
    ];
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.intra.42.fr/oauth/token")
        .form(&params)
        .send()
        .await;

    match response {
        Ok(res) => match res.status() {
            reqwest::StatusCode::OK => {
                let access_token: AccessToken = res.json().await?;
                Ok(access_token.access_token)
            }
            reqwest::StatusCode::UNAUTHORIZED => Err(SessionError::New(
                "Failed to generate access token. Please check your `config.toml` file.".into(),
            )),
            _ => panic!("uh oh! something unexpected happened"),
        },
        Err(e) => Err(SessionError::ReqwestError(e)),
    }
}

// Generate code grant token.
//
// # Example
//
// ```
// let session = Session::new(Some(Mode::Code)).await?;
// let access_token = generate_token(session.clone()).await?;
// ```
pub async fn generate_token(session: Session) -> Result<String, SessionError> {
    let client = BasicClient::new(
        ClientId::new(String::from(session.get_client_id())),
        Some(ClientSecret::new(String::from(session.get_client_secret()))),
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

    let ac_token = local_server(client).await?;
    Ok(ac_token)
}

// Creates local server with port number 8000 and waits for user to finish authorize.
async fn local_server(client: BasicClient) -> Result<String, SessionError> {
    let ac_token;
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
                    None => return Err(SessionError::NoneError),
                };
                let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;

                let code_pair = match url.query_pairs().find(|pair| {
                    let &(ref key, _) = pair;
                    key == "code"
                }) {
                    Some(code) => code,
                    None => return Err(SessionError::NoneError),
                };

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = match url.query_pairs().find(|pair| {
                    let &(ref key, _) = pair;
                    key == "state"
                }) {
                    Some(state) => state,
                    None => return Err(SessionError::NoneError),
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
            stream.write_all(response.as_bytes()).await?;

            debug!("42API returned the following code:\n{}\n", code.secret());
            debug!("42API returned the following state:\n{}\n", state.secret());

            // Exchange the code with a token.
            let token_res = client
                .exchange_code(code)
                .request_async(async_http_client)
                .await;
            let token = match token_res {
                Err(_) => return Err(SessionError::UnauthorizedResponse),
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
            ac_token = token.access_token().secret().to_owned();
            debug!("Access Token: {:?}", ac_token);
            debug!("42API returned the following scopes:\n{:?}\n", scopes);
            break;
        }
    }
    Ok(ac_token)
}
