use log::{debug, warn};
use anyhow::{Context, Result};
use reqwest::Response;
use reqwest::header::AUTHORIZATION;
use std::io::{self, BufRead};
use std::path::Path;
use std::{env};
use std::{fs, fs::File};

use crate::authorize::my_authorize;
use crate::authorize::token::TokenInfo;
use crate::structs::program::Program;

// pub async fn check_token_exist(prog: &mut Program) -> Result<(), Box<dyn error::Error>> {
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
            prog.access_token = my_authorize().await?;
            // write to .env file
            write_to_file(".env", format!("access_token={}", prog.access_token.to_owned()));
            prog.access_token.to_owned()
        }
    };
    prog.access_token = ac_token;
    // check_token(prog.access_token.to_owned(), prog).await?;
    Ok(())
}

async fn make_token_request(ac_token: String) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.intra.42.fr/oauth/token/info")
        .header(AUTHORIZATION, format!("Bearer {}", ac_token))
        .send()
        .await?;
    Ok(response)
}

pub async fn check_token_validity(ac_token: String, prog: &mut Program) -> Result<()> {
    let mut response = make_token_request(ac_token.to_owned()).await?;
    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("check_token(): OK");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("check_token(): UNAUTHORIZED");
            // token expired or wrong access token -> get new token
            prog.access_token = my_authorize().await?;
            update_file(prog.access_token.to_owned());
            response = make_token_request(prog.access_token.to_owned()).await?;
            match response.status() {
                reqwest::StatusCode::UNAUTHORIZED => panic!("Token validity check failed more than once"),
                reqwest::StatusCode::OK => (),
                _ => panic!("Uh oh! something unexpected happened."),
            }
        }
        _ => {
            warn!("check_token(): panic!");
            panic!("Uh oh! something unexpected happened.");
        }
    }

    prog.token = response.json::<TokenInfo>().await?;
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

    write!(file, "{}\n", content).unwrap();
}

fn update_file(token: String) {
    if let Ok(lines) = read_lines(".env") {
        for line in lines {
            if let Ok(ip) = line {
                let mut content = String::new();
                if ip.contains("access_token") {
                    content.push_str(format!("access_token={}", token).as_str());
                } else {
                    content.push_str(ip.as_str());
                }
                write_to_file(".temp", content);
            }
        }
    }
    fs::remove_file(".env").with_context(|| "Failed to remove .env file").unwrap();
    fs::rename(".temp", ".env").with_context(|| "Failed to rename .temp to .env").unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
