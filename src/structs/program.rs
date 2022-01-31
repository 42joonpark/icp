use crate::authorize::check;
use crate::authorize::my_authorize;
use crate::authorize::token;
use crate::structs::{campus, me};
use crate::CliError;
use log::{debug, info, warn};
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
    async fn get_access_token(&mut self) -> Result<(), CliError> {
        // self.access_token = Some(my_authorize(self.clone()).await?);
        self.access_token = Some(check::check_token_exist(self.clone()).await?);
        Ok(())
    }
    async fn check_token(&mut self) -> Result<String, CliError> {
        info!("check_token()");
        let mut update = false;
        let tok =
            check::check_token_validity(self.access_token.to_owned().unwrap_or_default()).await;
        // check_token_validity()ㅇ에서 현재 토큰이 유효한지만 체크하고, 만약에 인증되지 않은 상태면 다시 발급 받아야되.
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
        if update == true {
            info!("check_token(): update file");
            check::update_file(self.access_token.to_owned().unwrap_or_default())?;
        }
        self.token = Some(tok);
        self.access_token.to_owned().ok_or(CliError::NoneError)
    }
    async fn call(&mut self, uri: &str) -> Result<String, CliError> {
        info!("call()");
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

    // 매번 check_token_validity() 할 필요 없이 init() 할 떄 다 하는거야.
    // 1. ACCESS_TOKEN이 있으면 check_token_validity()로 유효한지 확인할 수 있고,
    // 2. ACCESS_TOKEN이 없으면 새로 만들 수 있지
    // 그러면 access_token을 얻는 함수를 따로 만들어야되.
    // 지금은 check_token_validity()에서 하는데 이걸 나눠준다.
    // 하는 역할을 분명하게 나누기.
    pub async fn init_program(&mut self) -> Result<(), CliError> {
        info!("init_program()");
        self.session = Some(Session {
            client_id: env::var("CLIENT_ID")?,
            client_secret: env::var("CLIENT_SECRET")?,
            access_token: None,
            token: None,
        });
        // self.session.get_access_token().await?;
        match self.session.as_mut() {
            Some(session) => session.get_access_token().await?,
            _ => (),
        }
        Ok(())
    }

    pub async fn with_session(&mut self, url: &str) -> Result<String, CliError> {
        info!("with_session()");
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
