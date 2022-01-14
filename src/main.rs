use reqwest::header::AUTHORIZATION;
use std::{error};
use log::{debug, warn};
use anyhow::{Result};
use dotenv;

pub mod authorize;
use authorize::{AccessToken, check};

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
	// let ac_token = my_authorize().await?;
	// info!("{}", format!("AccessToken: {}", ac_token));
	// run(ac_token).await?;

	let res = check::check_token_validity().await?;
	println!("{:?}", res);

	Ok(())
}