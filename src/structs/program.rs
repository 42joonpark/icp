use crate::authorize::check;
use crate::authorize::token;
use crate::structs::me;
use anyhow::{Context, Result};
use std::env;

#[derive(Default)]
pub struct Program {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub token: Option<token::TokenInfo>,
    pub me: me::Me,
    pub check_cnt: u8,
}

impl Program {
    pub fn new() -> Program {
        Program::default()
    }

    pub async fn init_program(&mut self) -> Result<()> {
        dotenv::dotenv().expect("Failed to read .env file!!");
        let client_id =
            env::var("client_id").with_context(|| "Failed to read `client_id`.".to_string())?;
        let client_secret =
            env::var("client_secret").with_context(|| "Failed to read `client_id`.".to_string())?;
        self.client_id = client_id;
        self.client_secret = client_secret;
        check::check_token_exist(self).await?;
        // check_token_validity not needed here.
        // main() -> welcome_msg() -> load_info() -> check_token_validity()
        // check::check_token_validity(self.access_token.to_owned(), self).await?;
        Ok(())
    }
}
