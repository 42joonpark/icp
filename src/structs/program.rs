use crate::authorize::check;
use crate::authorize::my_authorize;
use crate::authorize::token;
use crate::cli::Config;
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
    pub async fn new() -> Result<Self, CliError> {
        let path = "./config.toml";
        let content = fs::read_to_string(path)?;
        let mut session: Session = toml::from_str(&content)?;
        session.access_token = Some(check::check_token_exist(session.clone()).await?);
        Ok(session)
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

#[derive(Debug)]
pub struct Program {
    session: Option<Session>,
    pub config: Config,
}

impl Program {
    pub async fn new(config: Config) -> Result<Self, CliError> {
        info!("Program::new() Begin");
        let program = Program {
            session: Some(Session::new().await?),
            config,
        };
        info!("Program::new() End");
        Ok(program)
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
    async fn get_me(&mut self) -> Result<me::Me, CliError> {
        info!("get_me() Begin");
        let res = self.with_session("v2/me").await?;
        let me: me::Me = serde_json::from_str(res.as_str())?;
        info!("get_me() End");
        Ok(me)
    }

    pub async fn me(&mut self) -> Result<(), CliError> {
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

    pub async fn email(&mut self) -> Result<(), CliError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Email", m.email);
        Ok(())
    }

    pub async fn wallet(&mut self) -> Result<(), CliError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Wallet", m.wallet);
        Ok(())
    }

    pub async fn id(&mut self) -> Result<(), CliError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "ID", m.id);
        Ok(())
    }

    pub async fn login(&mut self) -> Result<(), CliError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Login", m.login);
        Ok(())
    }

    pub async fn correction_point(&mut self) -> Result<(), CliError> {
        let m = self.get_me().await?;
        println!("{:20}{}", "Correction point", m.correction_point);
        Ok(())
    }

    pub async fn campus(&mut self) -> Result<(), CliError> {
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
