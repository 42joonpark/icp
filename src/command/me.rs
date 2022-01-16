use std::{env, error};
use log::{debug, warn};
use anyhow::{Context, Result};
use reqwest::header::AUTHORIZATION;
use crate::structs::{program::Program};
use crate::authorize::check::check_token_validity;
use crate::make_json::jsonize;

pub async fn my_info(prog: &mut Program) -> Result<(), Box<dyn error::Error>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let client = reqwest::Client::new();
    let client_id =
        env::var("client_id").with_context(|| format!("Failed to read `client_id`."))?;
    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id.as_str()),
    ];

    check_token_validity(prog.access_token.to_owned(), prog).await?;
    let response = client
        .get("https://api.intra.42.fr/v2/me")
        .header(AUTHORIZATION, format!("Bearer {}", prog.access_token.to_owned()))
        .form(&params)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("my_info: reqwest OK");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("my_info: unauthorized!!");
        }
        _ => {
            panic!("uh oh! something unexpected happened.");
        }
    };

    let tmp = response.text().await?;
    prog.me = jsonize(tmp.as_str()).unwrap();
    debug!("{:#?}", prog.me);
    Ok(())
}