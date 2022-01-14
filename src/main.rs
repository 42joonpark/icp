use reqwest::header::AUTHORIZATION;
use serde::{Deserialize};
use std::{error, env};
use log::{debug, warn};
use anyhow::{Result};
use dotenv;

pub mod authorize;
pub mod info_struct;
use authorize::{AccessToken, check};
use info_struct::me::Me;

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

fn jsonize_vec<'a, T>(text: &'a str) -> Result<Vec<T>, serde_json::Error> 
    where T: Deserialize<'a>
{
    let camp: Vec<T> = serde_json::from_str(text)?;
    Ok(camp)
}

fn jsonize<'a, T>(text: &'a str) -> Result<T, serde_json::Error> 
    where T: Deserialize<'a>
{
    let camp: T = serde_json::from_str(text)?;
    Ok(camp)
}

async fn my_info() -> Result<(), Box<dyn error::Error>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let client = reqwest::Client::new();
    let client_id = env::var("client_id").unwrap();
    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id.as_str()),
    ];
	let access_token = env::var("access_token").unwrap();
    let response = client
        .get("https://api.intra.42.fr/v2/me")
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
		.form(&params)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            println!("ok~~");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("unauthorized!!");
        }
        _ => {
            panic!("uh oh! something unexpected happened.");
        }
    };

	let tmp = response.text().await?;
	let my_info: Me = jsonize(tmp.as_str()).unwrap();
	println!("{}", my_info.email);
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

	my_info().await?;

	Ok(())
}