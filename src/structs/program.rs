use crate::authorize::check;
use crate::authorize::token;
use crate::structs::{campus, me};
use crate::CliError;
use log::{debug, warn};
use reqwest::header::AUTHORIZATION;
use std::env;

#[derive(Clone, Default, Debug)]
pub struct Session {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: Option<String>,
    token: Option<token::TokenInfo>,
}

impl Session {
    // token을 check하는 건데 굳이 client_id까지 같이 줄 필요는 없지???
    // async fn check_token(&mut self) -> Result<(String, String), CliError> {
    //     let (ac_token, tok) = check::check_token_validity(self.clone()).await?;
    //     let client_id = self.client_id.to_owned();
    //     self.access_token = ac_token.to_owned();
    //     self.token = Some(tok);
    //     Ok((client_id.to_owned(), ac_token))
    // }
    async fn check_token(&mut self) -> Result<String, CliError> {
        let (ac_token, tok) = check::check_token_validity(self.clone()).await?;
        self.token = Some(tok);
        self.access_token = ac_token;
        self.access_token.to_owned().ok_or(CliError::NoneError)
    }
    async fn call(&mut self, uri: &str) -> Result<String, CliError> {
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
                return Err(CliError::UnauthorizedResult);
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
        self.session = Some(Session {
            client_id: env::var("CLIENT_ID")?,
            client_secret: env::var("CLIENT_SECRET")?,
            access_token: match env::var("ACCESS_TOKEN") {
                Ok(result) => Some(result),
                Err(_) => None,
            },
            token: None,
        });
        Ok(())
    }

    pub async fn with_session(&mut self, url: &str) -> Result<String, CliError> {
        let res = match &mut self.session {
            Some(session) => {
                let tmp = session.call(url).await?;
                tmp
            }
            None => return Err(CliError::SessionExistError),
        };
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
