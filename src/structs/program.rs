use std::env;
use anyhow::{Result, Context};
use crate::authorize::token;
use crate::authorize::check;
use crate::structs::me;

#[derive(Default)]
pub struct Program {
	pub client_id: String,
	pub client_secret: String,
	pub access_token: String,
	pub token: token::TokenInfo,
	pub me: me::Me,
	pub check_cnt: u8,
}

impl Program {
	pub fn new() -> Program {
		Program::default()
	}

	pub async fn init_program(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    	dotenv::dotenv().expect("Failed to read .env file!!");
    	let client_id =
        		env::var("client_id").with_context(|| format!("Failed to read `client_id`."))?;
    	let client_secret =
        		env::var("client_secret").with_context(|| format!("Failed to read `client_id`."))?;
		self.client_id = client_id;
		self.client_secret = client_secret;
		check::check_token_exist(self).await?;
		check::check_token_validity(self.access_token.to_owned(), self).await?;
		Ok(())
	}
}