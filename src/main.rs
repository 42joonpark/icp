// use std::{collections::HashMap, ops::Add};

use std::thread::AccessError;

use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use std::env;
use dotenv;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: i32,
    scope: String,
    created_at: i64,
}

async fn init_session() -> Result<AccessToken, reqwest::Error> {
    dotenv::dotenv().expect("Failed to read .env file");
    let client = reqwest::Client::new();
    let client_id = env::var("client_id").unwrap();
    let client_secret = env::var("client_secret").unwrap();
    let params = [
        ("grant_type", "client_credentials"),
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
            println!("ok~~");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("unauthorized!!");
        }
        _ => {
            panic!("uh oh! something unexpected happened.");
        }
    };
    let token = response.json::<AccessToken>().await?;
    Ok(token)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let acc_token = init_session().await?;

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.intra.42.fr/v2/cursus")
        .header(AUTHORIZATION, format!("Bearer {}", acc_token.access_token))
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
    let body = response.text().await?;
    println!("{}", body);

    Ok(())
}