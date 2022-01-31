use crate::structs::program::Session;
use crate::CliError;
use log::debug;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

// authorize with 42 OAuth2
pub async fn my_authorize(session: Session) -> Result<String, CliError> {
    let client_id = session.client_id.to_owned();
    let client_secret = session.client_secret.to_owned();
    let client = BasicClient::new(
        ClientId::new(client_id.to_owned()),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string())?,
        Some(TokenUrl::new(
            "https://api.intra.42.fr/oauth/token".to_string(),
        )?),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

    // generate OAuth2 url. set scope to public
    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public".to_string()))
        .url();

    // prints the authorize url
    println!("Browse to: {}", auth_url);

    // localhost:8080 server
    let ac_token = local_server(client).await?;
    Ok(ac_token)
}

// make local server localhost:8080 and waits for request and exchange access_token
async fn local_server(client: BasicClient) -> Result<String, CliError> {
    let mut ac_token = String::new();
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
                    None => return Err(CliError::NoneError),
                };
                let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;

                let code_pair = match url.query_pairs().find(|pair| {
                    let &(ref key, _) = pair;
                    key == "code"
                }) {
                    Some(code) => code,
                    None => return Err(CliError::NoneError),
                };

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = match url.query_pairs().find(|pair| {
                    let &(ref key, _) = pair;
                    key == "state"
                }) {
                    Some(state) => state,
                    None => return Err(CliError::NoneError),
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

            debug!("42API returned the following token:\n{:?}\n", token_res);

            if let Ok(token) = token_res {
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
            }
            // The server will terminate itself after collecting the first code.
            break;
        }
    }
    Ok(ac_token)
}

pub mod check;
pub mod token;
