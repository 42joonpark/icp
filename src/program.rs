use crate::cli::Config;
use log::info;
use ftapi::results::{campus, me};
use ftapi::Session;
use ftapi::SessionError;
use ftapi::token::*;

#[derive(Debug)]
pub struct Program {
    session: Option<Session>,
    pub config: Config,
}

impl Program {
    pub async fn new(config: Config) -> Result<Self, SessionError> {
        info!("Program::new() Begin");
        let program = Program {
            session: Some(Session::new("config.toml")?),
            config,
        };
        // Program이 access_token이 유효한지 확인을 하고 없으면 추가를 해줘야한다.
        info!("Program::new() End");
        Ok(program)
    }

    pub async fn with_session(&mut self, url: &str) -> Result<String, SessionError> {
        info!("with_session() Begin");
        let res = match &mut self.session {
            Some(session) => {
                let tmp = session.call(url).await?;
                tmp
            }
            None => return Err(SessionError::NoneError),
        };
        info!("with_session() End");
        Ok(res)
    }
}

impl Program {
    async fn get_me(&mut self) -> Result<me::Me, SessionError> {
        info!("get_me() Begin");
        let res = self.with_session("v2/me").await?;
        let me: me::Me = serde_json::from_str(res.as_str())?;
        info!("get_me() End");
        Ok(me)
    }

    pub async fn me(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        let title = if m.titles.is_empty() {
            ""
        } else {
            m.titles[0].name.split(' ').next().unwrap_or("")
        };
        println!("{} | {} {}", m.displayname, title, m.login);
        println!("{:20}{}", "Wallet", m.wallet);
        println!("{:20}{}", "Evaluation points", m.correction_point);
        println!("{:20}{}", "Cursus", m.cursus_users[1].cursus.name);
        Ok(())
    }

    pub async fn email(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Email", m.email);
        Ok(())
    }

    pub async fn wallet(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Wallet", m.wallet);
        Ok(())
    }

    pub async fn id(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "ID", m.id);
        Ok(())
    }

    pub async fn login(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Login", m.login);
        Ok(())
    }

    pub async fn correction_point(&mut self) -> Result<(), SessionError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Correction point", m.correction_point);
        Ok(())
    }

    pub async fn campus(&mut self) -> Result<(), SessionError> {
        let url = self.generate_url("v2/campus").await;
        let result = self.with_session(&url[..]).await?;
        let campuses: campus::Campus = serde_json::from_str(result.as_str())?;
        for camp in campuses {
            println!("{:#?}", camp);
        }
        Ok(())
    }

    // add url with config values.
    // if page exists than cat page... to url
    async fn generate_url(&mut self, url: &str) -> String {
        let mut res = String::new();
        res.push_str(url);
        res.push_str("?page=");
        if let Some(page) = &self.config.page {
            res.push_str(&page.to_string());
        }
        res
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
