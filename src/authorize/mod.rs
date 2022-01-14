use reqwest::header::AUTHORIZATION;
use serde::{Deserialize};
use std::ops::Add;
use std::{env, error, fmt, fs, fs::File};
use std::io::{self, BufRead};
use std::path::Path;
use log::{debug, warn};
use anyhow::{Context, Result};
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

use token::TokenInfo;

#[derive(Deserialize, Debug)]
pub struct AccessToken {
	pub access_token:	String,
	pub token_type:		String,
	pub expires_in:		i32,
	pub scope:			String,
	pub created_at:		i64,
}

impl AccessToken {
    pub fn new() -> AccessToken {
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

pub async fn my_authorize() -> Result<AccessToken, Box<dyn error::Error>> {
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
				ac_token.access_token = token.access_token().secret().to_owned();
				debug!("Access Token: {:?}", ac_token.access_token);
                debug!("42API returned the following scopes:\n{:?}\n", scopes);
            }

            // The server will terminate itself after collecting the first code.
            break;
        }
    }
	Ok(ac_token)
}

async fn check_now(ac_token: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.intra.42.fr/oauth/token/info")
        .header(AUTHORIZATION, format!("Bearer {}", ac_token))
        .send()
        .await
        .unwrap();
    
    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("check_now(): OK");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("check_now(): UNAUTHORIZED");
            // token expired or wrong access token
            // get new token
            let new_token = my_authorize().await?;
            update_file(new_token.access_token);
            return Ok(())
        }
        _ => {
            warn!("check_now(): panic!");
            panic!("Uh oh! something unexpected happened.");
        }
    }

    // let tmp = response.text().await?;
    let token_info = response.json::<TokenInfo>().await?;
    println!("{:?}", token_info);
    Ok(())
}

fn write_to_file(filename: &str, content: String) {
    use std::io::Write;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();
          
    write!(file, "{}\n", content).unwrap();
}

fn update_file(token: String) {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(".env") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                // println!("{}", ip);
                let mut content = String::new();
                if ip.contains("access_token") {
                    content.push_str(format!("access_token={}", token).as_str());
                } else {
                    content.push_str(ip.as_str());
                }
                write_to_file(".temp", content);
            }
        }
    }
    fs::remove_file(".env").unwrap();
    fs::rename(".temp", ".env").unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub async fn check_token_validity() -> Result<(), Box<dyn error::Error>> {
	dotenv::dotenv().expect("Failed to read .env file!!");
    let ac_token = env::var("access_token");
    let ac_token = match ac_token {
        Ok(content) =>  {
            debug!("check_token_validity(): found token");
            content
        }
        Err(_) => {
            debug!("check_token_validity(): token not found in .env file");
            let tok = my_authorize().await?;
            write_to_file(".env", format!("access_token={}", tok.access_token));
            // save to .env file
            return Ok(())
        }
    };
    check_now(ac_token).await?;
    Ok(())
}

pub mod token;