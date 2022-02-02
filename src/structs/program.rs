use crate::authorize::check;
use crate::authorize::my_authorize;
use crate::authorize::token;
use crate::structs::{campus, me};
use crate::CliError;
use log::{debug, info, warn};
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::fs;

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Session {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: Option<String>,
    token: Option<token::TokenInfo>,
}

impl Session {
    async fn get_access_token(&mut self) -> Result<(), CliError> {
        info!("get_access_token() Begin");
        self.access_token = Some(check::check_token_exist(self.clone()).await?);
        info!("get_access_token() End");
        Ok(())
    }
    /// check if token is valide.
    /// if not generate one.
    async fn check_token(&mut self) -> Result<String, CliError> {
        info!("check_token() Begin");
        let mut update = false;
        let tok =
            check::check_token_validity(self.access_token.to_owned().unwrap_or_default()).await;
        let tok = match tok {
            Ok(tok) => tok,
            Err(CliError::Unauthorized) => {
                self.access_token = Some(my_authorize(self.clone()).await?);
                update = true;
                check::check_token_validity(self.access_token.to_owned().unwrap_or_default())
                    .await?
            }
            Err(error) => {
                return Err(error);
            }
        };
        if update {
            info!("check_token(): update file");
            check::update_file(self.access_token.to_owned().unwrap_or_default())?;
        }
        info!("check_token() End");
        self.token = Some(tok);
        self.access_token.to_owned().ok_or(CliError::NoneError)
    }
    async fn call(&mut self, uri: &str) -> Result<String, CliError> {
        info!("call() Begin");
        let ac_token = self.check_token().await?;
        let client_id = self.client_id.to_owned();
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", client_id.as_str()),
        ];
        let response = client
            .get(format!("https://api.intra.42.fr/{}", uri))
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
                return Err(CliError::Unauthorized);
            }
            reqwest::StatusCode::FORBIDDEN => {
                warn!("call(): 402 FORBIDDEN ACCESS");
                return Err(CliError::Fobidden);
            }
            reqwest::StatusCode::NOT_FOUND => {
                warn!("404 NOT FOUND");
                return Err(CliError::NotFound);
            }
            _ => {
                panic!("uh oh! something unexpected happened");
            }
        }
        let tmp = response.text().await?;
        info!("call() End");
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

    pub async fn init_program(&mut self) -> Result<(), CliError> {
        info!("init_program() Begin");
        let client_info = fs::read_to_string("config.toml").unwrap();
        let client: Session = toml::from_str(client_info.as_str()).unwrap();
        self.session = Some(client);
        if let Some(session) = self.session.as_mut() {
            session.get_access_token().await?
        }
        info!("init_program() End");
        Ok(())
    }

    pub async fn with_session(&mut self, url: &str) -> Result<String, CliError> {
        info!("with_session() Begin");
        let res = match &mut self.session {
            Some(session) => {
                let tmp = session.call(url).await?;
                tmp
            }
            None => return Err(CliError::SessionExistError),
        };
        info!("with_session() End");
        Ok(res)
    }
}

impl Program {
    pub async fn email(&mut self) -> Result<String, CliError> {
        let result = self.with_session("v2/me").await?;
        let res: me::Me = serde_json::from_str(result.as_str())?;
        Ok(res.email)
    }

    pub async fn wallet(&mut self) -> Result<i64, CliError> {
        let result = self.with_session("v2/me").await?;
        let res: me::Me = serde_json::from_str(result.as_str())?;
        Ok(res.wallet)
    }

    pub async fn id(&mut self) -> Result<i64, CliError> {
        let result = self.with_session("v2/me").await?;
        let res: me::Me = serde_json::from_str(result.as_str())?;
        Ok(res.id)
    }

    pub async fn login(&mut self) -> Result<String, CliError> {
        let result = self.with_session("v2/me").await?;
        let res: me::Me = serde_json::from_str(result.as_str())?;
        Ok(res.login)
    }

    pub async fn correction_point(&mut self) -> Result<i64, CliError> {
        let result = self.with_session("v2/me").await?;
        let res: me::Me = serde_json::from_str(result.as_str())?;
        Ok(res.correction_point)
    }

    pub async fn campus(&mut self) -> Result<(), CliError> {
        let result = self.with_session("v2/campus").await?;
        let campuses: campus::Campus = serde_json::from_str(result.as_str())?;
        for camp in campuses {
            println!("{:#?}", camp);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_email() {
        let contents = fs::read_to_string("./return_value/me.json").unwrap();
        let my_info: me::Me = serde_json::from_str(contents.as_str()).unwrap();
        assert_eq!(my_info.email, "joonpark@student.42seoul.kr");
    }
}
