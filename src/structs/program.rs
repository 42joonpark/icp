use crate::authorize::check;
use crate::authorize::token;
use crate::structs::{campus, me};
use anyhow::{Context, Error, Result};
use log::{debug, warn};
use reqwest::header::AUTHORIZATION;
use std::env;

#[derive(Clone, Default, Debug)]
pub struct Session {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub access_token: Option<String>,
    token: Option<token::TokenInfo>,
}

impl Session {
    async fn check_token(&mut self) -> Result<(String, String)>{
        let (ac_token, tok) = check::check_token_validity(self.clone()).await?;
        let client_id = self.client_id.as_ref().unwrap();
        self.access_token = Some(ac_token.to_owned());
        self.token = Some(tok);
        Ok((client_id.to_owned(), ac_token))
    }
    async fn call(&mut self, uri: &str) -> Result<String> {
        let (client_id, ac_token) = self.check_token().await?;
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            // ("client_id", self.client_id.as_str()),
            ("client_id", client_id.as_str()),
        ];
        let response = client
            .get(format!("https://api.intra.42.fr/{}", uri))
            // .header(AUTHORIZATION, format!("Bearer {}", self.access_token))
            .header(AUTHORIZATION, format!("Bearer {}", ac_token))
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
    session: Option<Session>,
    pub check_cnt: u8,
}

impl Program {
    pub fn new() -> Program {
        Program::default()
    }

    pub async fn init_program(&mut self) -> Result<()> {
        self.session = Some(Session {
            client_id: Some(env::var("CLIENT_ID")
                .with_context(|| "Failed to read `client_id`.".to_string())?),
            client_secret: Some(env::var("CLIENT_SECRET")
                .with_context(|| "Failed to read `client_secret`.".to_string())?),
            access_token: None,
            token: None,
        });
        Ok(())
    }

    pub async fn with_session(&mut self, url: &str) -> Result<String> {
        let res = match &mut self.session {
            Some(session) => {
                let tmp = session.call(url).await?;
                tmp
            }
            None => {
                // TODO
                // any better way?
                return Err(Error::msg("")).with_context(|| "hafds");
            }
        };
        // let res = self.session.call(url).await?;
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
