use crate::authorize::check;
use crate::authorize::token;
use crate::structs::me;
use anyhow::{Context, Result, Error};
use reqwest::header::AUTHORIZATION;
use std::env;
use log::{debug, warn};

#[derive(Default, Debug)]
struct Session {
    client_id: String,
    client_secret: String,
    access_token: String,
    token: Option<token::TokenInfo>,
}

impl Session {
    fn renew(&self) -> String {
        String::new()
    }

    // TODO
    // 여기서 session을 &mut 하게 주고 있는데 
    // access token 변경 하려고 그랬는데... immutable하게 받을 방법은??
    async fn call(&mut self, uri: &str) -> Result<String> {
        // check token validity -> done.
        // if check fails return error 
        // API 호출하면... 얘를 그냥 str로 반환ㅎ해줘야해.
        // 그리고 호출한 함수에 돌아가서 맞는 모양으로 serde_json으로 바꿔줘야지.
        // 그럼 여기서는 그냥 String 반환.
        debug!("OLD ACCESS_TOKEN: {}", self.access_token);
        self.access_token = check::check_token_validity(self.access_token.to_owned()).await?;
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
                // TODO
                // Error 어떻게 반환할지 고민
                return Err(Error::msg("erro"));
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
    pub me: Option<me::Me>,
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
        self.session.client_secret =
            env::var("CLIENT_SECRET").with_context(|| "Failed to read `client_id`.".to_string())?;
        self.session.access_token = check::check_token_exist().await?;
        Ok(())
    }

    // 여기에 뭔가 하는 함수를 만들어야지
    // 내가 호출하려는 함수를 받아서
    // 토큰 검사를 하고 해당 하수를 호출하는 함수
    // 이렇게 하면 토큰 검사를 하는 부분은 여기 한 곳 밖에 없는거지
    // pub async fn with_session<F, R>(&mut self, f: F) -> Result<R> where F: Fn(&Session) -> R {
    //     match f(&self.session) {
    //         Ok(r) => r,
    //         Err(e) => match e {
    //             token_expired => {
    //                 self.session = self.session.renew(),
    //             }
    //         }
    //     }
    // }

    pub async fn with_session(&mut self, url: &str) -> Result<String> {
        let res = self.session.call(url).await?;
        // let result = serde_json::from_str(res.as_str())?;
        Ok(res)
    }
}

impl Program {
    // fn email(&self) {
    //     self.with_session(|session| async {
    //         let res = session.call("\\v2\\user").await?;
    //         let user = serde_json::from_str(res.as_str())?;
    //         Ok(user.wallet)
    //     });
    // }

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
}
