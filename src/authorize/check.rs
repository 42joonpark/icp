use reqwest::header::AUTHORIZATION;
use log::{debug, warn};
use std::{env, error};
use std::io::{self, BufRead};
use std::path::Path;
use std::{fs, fs::File};

use super::my_authorize;
use super::token::TokenInfo;

async fn check_now(ac_token: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.intra.42.fr/oauth/token/info")
        .header(AUTHORIZATION, format!("Bearer {}", ac_token))
        .send()
        .await
        .unwrap();
    
    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("check_now(): OK");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("check_now(): UNAUTHORIZED");
            // token expired or wrong access token
            // get new token
            let new_token = my_authorize().await?;
            update_file(new_token.access_token);
            return Ok(())
        }
        _ => {
            warn!("check_now(): panic!");
            panic!("Uh oh! something unexpected happened.");
        }
    }

    // let tmp = response.text().await?;
    let token_info = response.json::<TokenInfo>().await?;
    println!("{:?}", token_info);
    Ok(())
}

pub async fn check_token_validity() -> Result<(), Box<dyn error::Error>> {
	dotenv::dotenv().expect("Failed to read .env file!!");
    let ac_token = env::var("access_token");
    let ac_token = match ac_token {
        Ok(content) =>  {
            debug!("check_token_validity(): found token");
            content
        }
        Err(_) => {
            debug!("check_token_validity(): token not found in .env file");
            let tok = my_authorize().await?;
            // write to .env file
            write_to_file(".env", format!("access_token={}", tok.access_token));
            return Ok(())
        }
    };
    check_now(ac_token).await?;
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
    fs::remove_file(".env").unwrap();
    fs::rename(".temp", ".env").unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}