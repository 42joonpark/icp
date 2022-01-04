use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};
use serde_json::{self, to_writer_pretty, Map};
use std::{env, io::BufWriter, fs::File, collections::HashMap};
use dotenv;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct AccessToken {
    access_token:   String,
    token_type:     String,
    expires_in:     i32,
    scope:          String,
    created_at:     i64,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
struct Language {
    id:         u32,
    name:       String,
    identifier: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
struct Campus {
        id:                     serde_json::Value,
        name:                   serde_json::Value,
        time_zone:              serde_json::Value,
        language:               serde_json::Value,
        users_count:            serde_json::Value,
        vogsphere_id:           serde_json::Value,
        country:                serde_json::Value,
        address:                serde_json::Value,
        zip:                    serde_json::Value,
        city:                   serde_json::Value,
        website:                serde_json::Value,
        facebook:               serde_json::Value,
        twitter:                serde_json::Value,
        active:                 serde_json::Value,
        email_extension:        serde_json::Value,
        default_hidden_phone:   serde_json::Value,
        endpoint:               serde_json::Value,
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
    println!("ACCESSTOKEN: {}", token.access_token);
    Ok(token)
}

fn jsonize<'a, T>(text: &'a str) -> Result<Vec<T>, serde_json::Error> 
    where T: Deserialize<'a>
{
    let camp: Vec<T> = serde_json::from_str(text)?;
    Ok(camp)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let acc_token = init_session().await?;

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.intra.42.fr/v2/campus")
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

    let tmp = response.text().await?;
    let camp: Vec<Campus> = jsonize(tmp.as_str()).unwrap();
    let writer = BufWriter::new(File::create("res.json").unwrap());
    serde_json::to_writer_pretty(writer, &camp).unwrap();

    Ok(())
}
