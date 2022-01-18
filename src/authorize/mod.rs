use anyhow::{Context, Result};
use log::debug;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use std::{env};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

pub async fn my_authorize() -> Result<String> {
    dotenv::dotenv().expect("Failed to read .env file!!");
    let client_id =
        env::var("client_id").with_context(|| format!("Failed to read `client_id`."))?;
    let client_secret =
        env::var("client_secret").with_context(|| format!("Failed to read `client_secret`."))?;
    let client = BasicClient::new(
        ClientId::new(client_id.to_owned()),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string())?,
        Some(TokenUrl::new("https://api.intra.42.fr/oauth/token".to_string())?),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public".to_string()))
        .url();

    println!("Browse to: {}", auth_url);

    let mut ac_token = String::new();
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&mut stream);

                let mut request_line = String::new();
                // reader.read_line(&mut request_line).await.unwrap();
                reader.read_line(&mut request_line)
                        .await
                        .with_context(|| format!("Failed to read line"))?;

                let redirect_url = request_line
                                        .split_whitespace()
                                        .nth(1)
                                        .with_context(|| "Failed to parse request redirect url.")
                                        .unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url))
                                .with_context(|| "Failed to make redirect url")
                                .unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .with_context(|| "Failed to find code")
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .with_context(|| "Failed to find state")
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes())
                    .await
                    .with_context(|| "Failed to write HTTP response")
                    .unwrap();

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
