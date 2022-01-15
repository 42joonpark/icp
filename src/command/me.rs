use std::{env, error};
use reqwest::header::AUTHORIZATION;
use crate::structs::{program::Program, me::Me};
use crate::authorize::check::check_token_validity;
use crate::make_json::jsonize;

pub async fn my_info(prog: &mut Program) -> Result<Me, Box<dyn error::Error>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let client = reqwest::Client::new();
    let client_id = env::var("client_id").unwrap();
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
    Ok(my_info)
}