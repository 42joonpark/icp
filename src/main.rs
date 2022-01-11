use reqwest::header::AUTHORIZATION;
use std::{env, error, fmt};
use serde::{Deserialize};
use log::{debug, warn, info};
use anyhow::{Context, Result};
use dotenv;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

#[derive(Deserialize, Debug)]
struct AccessToken {
	access_token:	String,
	token_type:		String,
	expires_in:		i32,
	scope:			String,
	created_at:		i64,
}

impl AccessToken {
    fn new() -> AccessToken {
        AccessToken {
            access_token: String::new(),
            token_type: String::new(),
            expires_in: 0,
            scope: String::new(),
            created_at: 0,
        }
    }
}

impl fmt::Display for AccessToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n\tAccess Token:\t{}\n\tToken Type:\t{}\n\tExpires In:\t{}\n\tScope:\t\t{}\n\tCreated At:\t{}\n]", self.access_token, self.token_type, self.expires_in, self.scope, self.created_at)
    }
}

async fn init_session() -> Result<AccessToken, Box<dyn error::Error>> {
	dotenv::dotenv().expect("Failed to read .env file!!");
	let client = reqwest::Client::new();
	let client_id = env::var("client_id")
			.with_context(|| format!("Failed to read `client_id`."))?;
	let client_secret = env::var("client_secret")
			.with_context(|| format!("Failed to read `client_secret`."))?;
	let params = [
        ("grant_type", "authorization_code"),
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
	];
	let response = client
		.post("https://api.intra.42.fr/oauth/token")
		.form(&params)
		.send()
		.await
		.unwrap();
	match response.status() {
		reqwest::StatusCode::OK => {
			debug!("init_session(): oauth token generated.!");
		}
		reqwest::StatusCode::UNAUTHORIZED => {
			warn!("Unauthorized client info.");
		}
		_ => {
			panic!("Uh Oh! Something unexpected happened.");
		}
	};
	let token = response.json::<AccessToken>().await
			.with_context(|| format!("Failed to json access token."))?;
	Ok(token)
}

async fn check_login() -> Result<AccessToken, Box<dyn error::Error>> {
	let at = init_session().await;
	match at {
		Err(error) => {
			warn!("check_login(): check .env file.");
			Err(error)
		}
		Ok(content) => {
			debug!("check_login(): AccessToken generated.");
			Ok(content)
		}
	}
}

async fn run(ac_token: AccessToken) -> Result<(), Box<dyn error::Error>> {
	dotenv::dotenv().expect("Failed to read .env file!!");
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.intra.42.fr/v2/campus/29/users")
        .header(AUTHORIZATION, format!("Bearer {}", ac_token.access_token))
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
			debug!("run(): StatusCode::OK.");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("unauthorized!!");
        }
        _ => {
            panic!("uh oh! something unexpected happened.");
        }
    };

	let tmp = response.text().await?;
	println!("{}", tmp);

	Ok(())
}

async fn authorize() -> Result<AccessToken, Box<dyn error::Error>> {
	dotenv::dotenv().expect("Failed to read .env file!!");
	let client_id = env::var("client_id")
			.with_context(|| format!("Failed to read `client_id`."))?;
	let client_secret = env::var("client_secret")
		.with_context(|| format!("Failed to read `client_secret`."))?;
	let client =
	BasicClient::new(
		ClientId::new(client_id.to_owned()),
	    Some(ClientSecret::new(client_secret)),
		AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string())?,
		Some(TokenUrl::new("https://api.intra.42.fr/oauth/token".to_string())?)
	)
	.set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);

	let (auth_url, csrf_token) = client
	.authorize_url(CsrfToken::new_random)
	.add_scope(Scope::new("public".to_string()))
	.url();

	println!("Browse to: {}", auth_url);

	let mut ac_token = AccessToken::new();
	let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&mut stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).await.unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
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
            stream.write_all(response.as_bytes()).await.unwrap();

            /*
            println!("42API returned the following code:\n{}\n", code.secret());
            println!(
                "42API returned the following state:\n{}\n",
                state.secret(),
            );
            */

            // Exchange the code with a token.
            let token_res = client
                .exchange_code(code)
                .request_async(async_http_client)
                .await;

            println!("42API returned the following token:\n{:?}\n", token_res);

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
				ac_token.access_token = token.access_token().secret().to_owned();
				info!("Access Token: {:?}", ac_token.access_token);
                println!("42API returned the following scopes:\n{:?}\n", scopes);
            }

            // The server will terminate itself after collecting the first code.
            break;
        }
    }
	Ok(ac_token)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>>{
	env_logger::init();
	authorize().await?;
	// let ac_token = check_login().await?;
	// info!("{}", format!("AccessToken: {}", ac_token));
	// run(ac_token).await?;

	Ok(())
}