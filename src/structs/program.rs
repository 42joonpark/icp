use crate::authorize::token::TokenInfo;
use crate::authorize::check;

pub struct Program {
	pub client_id: String,
	pub client_secret: String,
	pub access_token: String,
	pub token: TokenInfo,
	pub check_cnt: u8,
}

impl Program {
	pub fn new() -> Program {
		Program {
			client_id: String::from(""),
			client_secret: String::from(""),
			access_token: String::from(""),
			token: TokenInfo::new(),
			check_cnt: 0,
		}
	}
	pub async fn init_program(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    	dotenv::dotenv().expect("Failed to read .env file!!");
		let client_id = std::env::var("client_id").unwrap();
		let client_secret = std::env::var("client_secret").unwrap();
		self.client_id = client_id;
		self.client_secret = client_secret;
		check::check_token_exist(self).await?;
		check::check_token_validity(self.access_token.to_owned(), self).await?;
		Ok(())
	}
}