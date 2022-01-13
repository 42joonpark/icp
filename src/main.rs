use reqwest::header::AUTHORIZATION;
use std::{error};
use log::{debug, warn};
use anyhow::{Context, Result};
use dotenv;

pub mod authorize;
use authorize::{AccessToken, my_authorize};

/*
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
*/

async fn run(ac_token: AccessToken) -> Result<(), Box<dyn error::Error>> {
	dotenv::dotenv().expect("Failed to read .env file!!");
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.intra.42.fr/v2/me")
        .header(AUTHORIZATION, format!("Bearer {}", ac_token.access_token))
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
			debug!("run(): StatusCode::OK.");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("unauthorized!!");
        }
        _ => {
            panic!("uh oh! something unexpected happened.");
        }
    };

	let tmp = response.text().await?;
	println!("{}", tmp);

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>>{
	env_logger::init();
	let ac_token = my_authorize().await?;
	// info!("{}", format!("AccessToken: {}", ac_token));
	run(ac_token).await?;

	Ok(())
}