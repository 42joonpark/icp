use crate::authorize::check::check_token_validity;
use crate::json::jsonize;
use crate::structs::program::Program;
use anyhow::{Context, Result};
use log::{debug, warn};
use reqwest::header::AUTHORIZATION;
use std::env;

// request /v2/me
pub async fn load_info(prog: &mut Program) -> Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let client = reqwest::Client::new();
    let client_id =
        env::var("client_id").with_context(|| "Failed to read `client_id`.".to_string())?;
    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id.as_str()),
    ];

    check_token_validity(prog.access_token.to_owned(), prog).await?;
    let response = client
        .get("https://api.intra.42.fr/v2/me")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", prog.access_token.to_owned()),
        )
        .form(&params)
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("load_info: reqwest OK");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("load_info: unauthorized!!");
        }
        _ => {
            panic!("uh oh! something unexpected happened.");
        }
    };

    let tmp = response.text().await?;
    prog.me = jsonize(tmp.as_str()).unwrap();
    // debug!("{:#?}", prog.me);
    Ok(())
}
