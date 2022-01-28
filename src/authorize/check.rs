use crate::authorize::my_authorize;
use crate::authorize::token;
use crate::structs::program::Session;
use crate::CliError;
use log::{debug, warn};
use reqwest::header::AUTHORIZATION;
use reqwest::Response;
use std::env;
use std::{
    io::{self, BufRead},
    path::Path,
    {fs, fs::File},
};

// teturn access_token
// if not exist in .env then create new access_token
pub async fn check_token_exist(session: Session) -> Result<String, CliError> {
    let ac_token = env::var("ACCESS_TOKEN");
    let ac_token = match ac_token {
        Ok(content) => {
            debug!("check_token_validity(): found token");
            content
        }
        Err(_) => {
            debug!("check_token_validity(): token not found in .env file");
            // if access_token does not exist, than generate access_token
            let tmp = my_authorize(session).await?;
            // write to .env file
            write_to_file(".env", format!("ACCESS_TOKEN={}", tmp));
            tmp
        }
    };
    Ok(ac_token)
}

async fn token_info_request(ac_token: String) -> Result<Response, CliError> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.intra.42.fr/oauth/token/info")
        .header(AUTHORIZATION, format!("Bearer {}", ac_token))
        .send()
        .await?;
    Ok(response)
}

// check if current access token is valide.
// if not generate new access token
pub async fn check_token_validity(
    session: Session,
) -> Result<(String, token::TokenInfo), CliError> {
    let mut ac_token = match &session.access_token {
        Some(x) => x.to_owned(),
        None => String::new(),
    };
    let mut response = token_info_request(ac_token.to_owned()).await?;
    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("check_token(): OK");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("check_token(): UNAUTHORIZED");
            // token expired or wrong access token -> generate new token
            ac_token = my_authorize(session).await?;
            // update .env file with new access token
            update_file(ac_token.to_owned())?;
            // make request again to check if token is valide
            response = token_info_request(ac_token.to_owned()).await?;
            match response.status() {
                reqwest::StatusCode::UNAUTHORIZED => {
                    todo!("try not to panic here. When with wrong client_secret this happens");
                    // panic!("Token validity check failed more than once")
                }
                reqwest::StatusCode::OK => (),
                _ => todo!("try not to panic here"), // _ => panic!("Uh oh! something unexpected happened."),
            }
        }
        _ => {
            warn!("check_token(): panic!");
            todo!("try not to panic here");
            // panic!("Uh oh! something unexpected happened.");
        }
    }
    let res = response.text().await?;
    let tok: token::TokenInfo = serde_json::from_str(res.as_str())?;
    Ok((ac_token, tok))
}

fn write_to_file(filename: &str, content: String) {
    use std::io::Write;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();

    writeln!(file, "{}", content).unwrap();
}

fn update_file(token: String) -> Result<(), CliError> {
    if let Ok(lines) = read_lines(".env") {
        for line in lines.flatten() {
            let mut content = String::new();
            if line.contains("ACCESS_TOKEN") {
                content.push_str(format!("ACCESS_TOKEN={}", token).as_str());
            } else {
                content.push_str(line.as_str());
            }
            write_to_file(".temp", content);
        }
    }
    fs::remove_file(".env")?;
    fs::rename(".temp", ".env")?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
