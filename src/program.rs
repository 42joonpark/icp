use crate::cli::Config;
use cli_42::results::*;
use cli_42::token::TokenInfo;
use cli_42::Mode;
use cli_42::Session;
use cli_42::SessionError;
use url::Url;

#[derive(Debug)]
pub struct Program {
    session: Session,
    pub token: Option<TokenInfo>,
    pub config: Config,
}

impl Program {
    pub async fn new(config: Config) -> Result<Self, SessionError> {
        let program = Program {
            session: Session::new(Some(Mode::Credentials)).await?,
            token: None,
            config,
        };
        Ok(program)
    }

    pub async fn call(&mut self, url: &str) -> Result<String, SessionError> {
        let res = self.session.call(url).await?;
        Ok(res)
    }
}

impl Program {
    async fn get_user_with_login(&mut self) -> Result<user::UserElement, SessionError> {
        let url = "https://api.intra.42.fr/v2/users";
        let url = Url::parse_with_params(
            url,
            &[
                ("client_id", self.session.get_client_id()),
                ("filter[login]", self.session.get_login()),
            ],
        )?;

        let res = self.call(url.as_str()).await?;
        let user: user::User = serde_json::from_str(res.as_str())?;
        Ok(user[0].clone())
    }

    async fn get_me(&mut self, id: i64) -> Result<me::Me, SessionError> {
        let url = format!("https://api.intra.42.fr/v2/users/{}", id);
        let url = Url::parse_with_params(&url, &[("client_id", self.session.get_client_id())])?;

        let res = self.call(url.as_str()).await?;
        let me: me::Me = serde_json::from_str(res.as_str())?;
        Ok(me)
    }

    pub async fn me(&mut self) -> Result<(), SessionError> {
        let tmp = self.get_user_with_login().await?;
        let id = tmp.id;
        let m = self.get_me(id).await?;
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
        let tmp = self.get_user_with_login().await?;
        let id = tmp.id;
        let m = self.get_me(id).await?;
        println!("{:20}{}", "Email", m.email);
        Ok(())
    }

    /*
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
    */
}
