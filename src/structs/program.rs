use crate::authorize::check;
use crate::authorize::token;
use crate::structs::{campus, me};
use anyhow::{Context, Error, Result};
use log::{debug, warn};
use reqwest::header::AUTHORIZATION;
use std::env;

#[derive(Default, Debug)]
struct Session {
    client_id: String,
    client_secret: String,
    access_token: String,
    token: Option<token::TokenInfo>,
}

impl Session {
    async fn call(&mut self, uri: &str) -> Result<String> {
        debug!("OLD ACCESS_TOKEN: {}", self.access_token);
        let (ac_token, tok) = check::check_token_validity(self.access_token.to_owned()).await?;
        self.access_token = ac_token;
        self.token = Some(tok);
        debug!("NEW ACCESS_TOKEN: {}", self.access_token);
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", self.client_id.as_str()),
        ];
        let response = client
            .get(format!("https://api.intra.42.fr/{}", uri))
            .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .form(&params)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                debug!("call(): reqwest OK");
            }
            reqwest::StatusCode::UNAUTHORIZED => {
                warn!("call(): unauthorized");
                return Err(Error::msg("Unauthoized."));
            }
            _ => {
                panic!("uh oh! something unexpected happened");
            }
        }
        let tmp = response.text().await?;
        Ok(tmp)
    }
}

#[derive(Default, Debug)]
pub struct Program {
    session: Session,
    pub check_cnt: u8,
}

impl Program {
    pub fn new() -> Program {
        Program::default()
    }

    pub async fn init_program(&mut self) -> Result<()> {
        dotenv::dotenv()?;
        self.session.client_id =
            env::var("CLIENT_ID").with_context(|| "Failed to read `client_id`.".to_string())?;
        self.session.client_secret = env::var("CLIENT_SECRET")
            .with_context(|| "Failed to read `client_secret`.".to_string())?;
        self.session.access_token = check::check_token_exist().await?;
        Ok(())
    }

    pub async fn with_session(&mut self, url: &str) -> Result<String> {
        let res = self.session.call(url).await?;
        Ok(res)
    }
}

impl Program {
    pub async fn email(&mut self) -> Result<String> {
        let result = self.with_session("v2/me").await?;
        let res: me::Me = serde_json::from_str(result.as_str())?;
        Ok(res.email)
    }

    pub async fn wallet(&mut self) -> Result<i64> {
        let result = self.with_session("v2/me").await?;
        let res: me::Me = serde_json::from_str(result.as_str())?;
        Ok(res.wallet)
    }

    pub async fn id(&mut self) -> Result<i64> {
        let result = self.with_session("v2/me").await?;
        let res: me::Me = serde_json::from_str(result.as_str())?;
        Ok(res.id)
    }

    pub async fn login(&mut self) -> Result<String> {
        let result = self.with_session("v2/me").await?;
        let res: me::Me = serde_json::from_str(result.as_str())?;
        Ok(res.login)
    }

    pub async fn correction_point(&mut self) -> Result<i64> {
        let result = self.with_session("v2/me").await?;
        let res: me::Me = serde_json::from_str(result.as_str())?;
        Ok(res.correction_point)
    }

    pub async fn campus(&mut self) -> Result<()> {
        let result = self.with_session("v2/campus").await?;
        let campuses: campus::Campus = serde_json::from_str(result.as_str())?;
        for camp in campuses {
            println!("{:#?}", camp);
        }
        Ok(())
    }
}
