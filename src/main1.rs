use serde::{Deserialize};
use std::process::Command;
use std::{env, error, fmt};
use std::io::{self, Write};
use log::{warn, debug, info};
use anyhow::{Context, Result, Error};
use dotenv;

#[derive(Deserialize, Debug)]
struct AccessToken {
	access_token:	String,
	token_type:		String,
	expires_in:		i32,
	scope:			String,
	created_at:		i64,
}

impl fmt::Display for AccessToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n\tAccess Token:\t{}\n\tToken Type:\t{}\n\tExpires In:\t{}\n\tScope:\t\t{}\n\tCreated At:\t{}\n]", self.access_token, self.token_type, self.expires_in, self.scope, self.created_at)
    }
}

async fn init_session() -> Result<AccessToken, Box<dyn error::Error>> {
	dotenv::dotenv().expect("Failed to read .env file!!");
	let client = reqwest::Client::new();
	let client_id = env::var("client_id")
			.with_context(|| format!("Failed to read `client_id`."))?;
	let client_secret = env::var("client_secret")
			.with_context(|| format!("Failed to read `client_secret`."))?;
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
			debug!("init_session(): oauth token generated.!");
		}
		reqwest::StatusCode::UNAUTHORIZED => {
			warn!("Unauthorized client info.");
		}
		_ => {
			panic!("Uh Oh! Something unexpected happened.");
		}
	};
	let token = response.json::<AccessToken>().await
			.with_context(|| format!("Failed to json access token."))?;
	Ok(token)
}

async fn check_login() -> Result<AccessToken, Box<dyn error::Error>> {
	let at = init_session().await;
	match at {
		Err(error) => {
			warn!("check_login(): check .env file.");
			Err(error)
		}
		Ok(content) => {
			debug!("check_login(): AccessToken generated.");
			Ok(content)
		}
	}
}

async fn run() -> Result<(), Box<dyn error::Error>> {
	let reader = io::stdin();
	loop {
		let mut line = String::new();	// string used for input
		print!("42_cli > ");
		io::stdout().flush()?;					// without this "42_cli > " does not appear.
		let bytes = reader.read_line(&mut line);
		let bytes = match bytes {			// unwrap Result<>
			Err(error) => {
				eprintln!("{}", error);
				continue;
			}
			Ok(content) => {
				content
			}
		};
		let command = line.to_uppercase();	// make line to uppercase. ex) 'quit', 'QuIt', 'qUit' -> treat all same.
		match command.as_str() {
			 "WALLET" => {
				println!("Hello");
			}
			"US" => {
				println!("US");
			}
			"Rust" => {
				println!("Rust");
			}
			"Python" => {
				println!("Python");
			}
			_ => {
				println!("Print something");
			}
		}
		if bytes == 0 || line.contains("quit") {
			println!("bye!");
			break ;
		}
		println!("{}", line);
	}
	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
	env_logger::init();
	let ac_token = check_login().await?;
	info!("{}", format!("AccessToken: {}", ac_token));
	run().await?;

	Ok(())
}