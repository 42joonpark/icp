use anyhow::Result;
use dotenv;
use log::debug;
use reqwest::header::AUTHORIZATION;
use std::io::{self, Write};
use std::{env, error};

pub mod authorize;
pub mod structs;
pub mod make_json;
use structs::program::Program;
use structs::me::Me;
use make_json::*;

async fn run(prog: &mut Program) -> Result<(), Box<dyn error::Error>> {
    let reader = io::stdin();
    loop {
        let mut line = String::new();
        print!("42_cli > ");
        io::stdout().flush()?;
        let bytes = reader.read_line(&mut line);
        match bytes {
            // unwrap Result<>
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
            Ok(size) => {
                if size == 0 {
                    println!("bye!");
                    break;
                }
            }
        }
        let command = line.trim().to_uppercase();
        debug!("COMMAND: {}", command);
        match command.as_str() {
            "ME" => {
                my_info(prog).await?;
            }
            "HELP" | "COMMAND" => {}
            "QUIT" => {
                println!("bye!!!");
                break;
            }
            _ => {}
        }
        println!("{}", line);
    }
    Ok(())
}

async fn my_info(prog: &mut Program) -> Result<(), Box<dyn error::Error>> {
    dotenv::dotenv().expect("Failed to read .env file");
    let client = reqwest::Client::new();
    let client_id = env::var("client_id").unwrap();
    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id.as_str()),
    ];
    // let access_token = env::var("access_token").unwrap();
    let access_token = prog.access_token.to_owned();
    let response = client
        .get("https://api.intra.42.fr/v2/me")
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
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
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    env_logger::init();
    let mut program = Program::new();
    program.init_program().await?;

    // let res = check::check_token_validity().await?;
    // println!("{:?}", res);

    run(&mut program).await?;
    Ok(())
}
