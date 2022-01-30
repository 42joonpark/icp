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
            write_to_file(".env", format!("ACCESS_TOKEN={}", tmp))?;
            tmp
        }
    };
    Ok(ac_token)
}

// 여기서는 ac_token이 항상 필요하기 때문에 Option으로 들어오면 안되
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
) -> Result<(Option<String>, token::TokenInfo), CliError> {
    // ac_token은 만약에 첫 호출이라면 None이 올 수 있다.
    let mut ac_token: Option<String> = session.access_token.as_ref().map(|x| x.to_owned());
    let mut response =
        token_info_request(ac_token.as_ref().map(|x| x.to_owned()).unwrap_or_default()).await?;
    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("check_token(): OK");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("check_token(): UNAUTHORIZED");
            // token expired or wrong access token -> generate new token
            ac_token = Some(my_authorize(session).await?);
            // update .env file with new access token
            update_file(ac_token.as_ref().map(|x| x.to_owned()).unwrap_or_default())?;
            // make request again to check if token is valide
            response =
                token_info_request(ac_token.as_ref().map(|x| x.to_owned()).unwrap_or_default())
                    .await?;
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

fn write_to_file(filename: &str, content: String) -> Result<(), CliError> {
    use std::io::Write;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename)?;
    writeln!(file, "{}", content).unwrap();
    Ok(())
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
            write_to_file(".temp", content)?;
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
