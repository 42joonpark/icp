use anyhow::{Context, Result};
use log::{debug, warn};
use reqwest::header::AUTHORIZATION;
use reqwest::Response;
use std::env;
use std::{
    io::{self, BufRead},
    path::Path,
    {fs, fs::File},
};

use crate::authorize::my_authorize;
use crate::authorize::token::TokenInfo;
use crate::structs::program::Program;

// check if access_token exists inside .env file
pub async fn check_token_exist(prog: &mut Program) -> Result<()> {
    dotenv::dotenv().expect("Failed to read .env file!!");
    let ac_token = env::var("access_token");
    let ac_token = match ac_token {
        Ok(content) => {
            debug!("check_token_validity(): found token");
            content
        }
        Err(_) => {
            debug!("check_token_validity(): token not found in .env file");
            // if access_token does not exist, than generate access_token
            prog.access_token = my_authorize().await?;
            // write to .env file
            write_to_file(
                ".env",
                format!("access_token={}", prog.access_token.to_owned()),
            );
            // duplicate ...
            prog.access_token.to_owned()
        }
    };
    prog.access_token = ac_token;
    Ok(())
}

async fn token_info_request(ac_token: String) -> Result<Response, reqwest::Error> {
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
pub async fn check_token_validity(ac_token: String, prog: &mut Program) -> Result<()> {
    let mut response = token_info_request(ac_token.to_owned()).await?;
    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("check_token(): OK");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("check_token(): UNAUTHORIZED");
            // token expired or wrong access token -> generate new token
            prog.access_token = my_authorize().await?;
            // update .env file with new access token
            update_file(prog.access_token.to_owned());
            // make request again to check if token is valide
            response = token_info_request(prog.access_token.to_owned()).await?;
            match response.status() {
                reqwest::StatusCode::UNAUTHORIZED => {
                    panic!("Token validity check failed more than once")
                }
                reqwest::StatusCode::OK => (),
                _ => panic!("Uh oh! something unexpected happened."),
            }
        }
        _ => {
            warn!("check_token(): panic!");
            panic!("Uh oh! something unexpected happened.");
        }
    }

    // get token info
    prog.token = Some(response.json::<TokenInfo>().await?);
    debug!("{:?}", prog.token);
    debug!("{:?}", prog.client_id);
    debug!("{:?}", prog.client_secret);
    debug!("{:?}", prog.access_token);
    Ok(())
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

fn update_file(token: String) {
    if let Ok(lines) = read_lines(".env") {
        for line in lines.flatten() {
            let mut content = String::new();
            if line.contains("access_token") {
                content.push_str(format!("access_token={}", token).as_str());
            } else {
                content.push_str(line.as_str());
            }
            write_to_file(".temp", content);
        }
    }
    fs::remove_file(".env")
        .with_context(|| "Failed to remove .env file")
        .unwrap();
    fs::rename(".temp", ".env")
        .with_context(|| "Failed to rename .temp to .env")
        .unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
