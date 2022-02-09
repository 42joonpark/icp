use crate::authorize::my_authorize;
use crate::authorize::token;
use crate::error::CliError;
use crate::structs::program::Session;
use log::{debug, info, warn};
use reqwest::header::AUTHORIZATION;
use reqwest::Response;
use std::{
    io::{self, BufRead},
    path::Path,
    {fs, fs::File},
};

/// check if session's access_token is not None.
/// if None then try to generate one
/// if generated then write to `config.toml`
pub async fn check_token_exist(session: Session) -> Result<String, CliError> {
    info!("check_token_exist() Begin");
    let ac_token = match session.access_token {
        Some(token) => {
            info!("check_token_exist(): found token");
            token
        }
        None => {
            debug!("check_token_exist(): token not found in ./config.toml file");
            let tmp = my_authorize(session).await?;
            write_to_file("config.toml", format!("\naccess_token=\"{}\"", tmp))?;
            tmp
        }
    };
    info!("check_token_exist() End");
    Ok(ac_token)
}

/// get current access_token information.
async fn token_info_request(ac_token: String) -> Result<Response, CliError> {
    info!("token_info_request() Begin");
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.intra.42.fr/oauth/token/info")
        .header(AUTHORIZATION, format!("Bearer {}", ac_token))
        .send()
        .await?;
    info!("token_info_request() END");
    Ok(response)
}

/// check if current access token is valide.
pub async fn check_token_validity(ac_token: String) -> Result<token::TokenInfo, CliError> {
    info!("check_token_validity() Begin");
    let response = token_info_request(ac_token.to_owned()).await?;
    match response.status() {
        reqwest::StatusCode::OK => {
            info!("check_token_validity(): reqwest::StatusCode::OK");
            debug!("check_token(): OK");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("check_token_validity(): UNAUTHORIZED");
            return Err(CliError::Unauthorized);
        }
        reqwest::StatusCode::FORBIDDEN => {
            warn!("check_token_validity(): 402 FORBIDDEN ACCESS");
            return Err(CliError::Fobidden);
        }
        reqwest::StatusCode::NOT_FOUND => {
            warn!("check_token_validity(): 404 NOT FOUND");
            return Err(CliError::NotFound);
        }
        _ => {
            warn!("check_token(): panic!");
            todo!("try not to panic here");
            // panic!("Uh oh! something unexpected happened.");
        }
    }
    let res = response.text().await?;
    let tok: token::TokenInfo = serde_json::from_str(res.as_str())?;
    info!("check_token_validity() End");
    Ok(tok)
}

/// write content to file
fn write_to_file(filename: &str, content: String) -> Result<(), CliError> {
    use std::io::Write;

    info!("write_to_file()");
    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)?;
    writeln!(file, "{}", content).unwrap();
    Ok(())
}

/// update access_token inside config.toml
pub fn update_file(token: String) -> Result<(), CliError> {
    info!("update_file()");
    if let Ok(lines) = read_lines("config.toml") {
        for line in lines.flatten() {
            let mut content = String::new();
            if line.contains("access_token") {
                content.push_str(format!("access_token=\"{}\"", token).as_str());
            } else {
                content.push_str(line.as_str());
            }
            write_to_file(".temp", content)?;
        }
    }
    fs::remove_file("config.toml")?;
    fs::rename(".temp", "config.toml")?;
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
