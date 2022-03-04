use crate::cli::Cli;
use crate::client::Client;
use crate::error::CliError;
use crate::results::campus_event;
use crate::results::me::Me;
use crate::results::me::User;
use crate::results::me::UserElement;
use crate::results::slots::Slots;
use crate::session;

use chrono::{DateTime, Duration, Local, Utc};
use url::Url;

pub struct Program {
    _client: Client,
    _config: Cli,
}

impl Program {
    pub fn new(_client: Client, _config: Cli) -> Self {
        Self { _client, _config }
    }

    pub fn config(&self) -> &Cli {
        &self._config
    }

    pub async fn run(&self) -> Result<(), CliError> {
        let command = self._config._command.trim().to_lowercase();
        match command.as_str() {
            "me" => self.me().await?,
            "event" => self.event().await?,
            "email" => self.email().await?,
            "projects" => self.projects().await?,
            "slot" => self.print_slots().await?,
            _ => println!("{} is not a valid command", command),
        }
        Ok(())
    }
}

// functions for the "me" command
impl Program {
    async fn get_user(&self) -> Result<Me, CliError> {
        if self._config.user().is_empty() {
            Ok(self.get_me().await?)
        } else {
            Ok(self.get_user_with_id().await?)
        }
    }

    async fn get_me(&self) -> Result<Me, CliError> {
        let uri = "https://api.intra.42.fr/v2/me";
        let uri = Url::parse_with_params(uri, &[("client_id", self._client.client_id())])?;
        let res = session::call(
            self._client.access_token(),
            self._client.client_id(),
            uri.as_str(),
        )
        .await?;
        Ok(serde_json::from_str(res.as_str())?)
    }

    async fn get_user_with_login(&self) -> Result<UserElement, CliError> {
        let uri = "https://api.intra.42.fr/v2/users";
        let uri = Url::parse_with_params(
            uri,
            &[
                ("client_id", self._client.client_id()),
                ("filter[login]", &self._config.user()),
            ],
        )?;
        let res = session::call(
            self._client.access_token(),
            self._client.client_id(),
            uri.as_str(),
        )
        .await?;
        let user: User = serde_json::from_str(res.as_str())?;
        if user.is_empty() {
            return Err(CliError::UserNotFound(self._config.user()));
        }
        Ok(user[0].clone())
    }

    async fn get_user_with_id(&self) -> Result<Me, CliError> {
        let user = self.get_user_with_login().await?;
        let uri = format!("https://api.intra.42.fr/v2/users/{}", user.id);
        let uri = Url::parse_with_params(&uri, &[("client_id", self._client.client_id())])?;
        let res = session::call(
            self._client.access_token(),
            self._client.client_id(),
            uri.as_str(),
        )
        .await?;
        Ok(serde_json::from_str(res.as_str())?)
    }
}

// functions for the slot command
impl Program {
    async fn get_slots(&self) -> Result<Slots, CliError> {
        let uri = "https://api.intra.42.fr/v2/me/slots";
        let uri = Url::parse_with_params(uri, &[("client_id", self._client.client_id())])?;
        let res = session::call(
            self._client.access_token(),
            self._client.client_id(),
            uri.as_str(),
        )
        .await?;
        // println!("{:#?}", res);
        Ok(serde_json::from_str(res.as_str())?)
    }

    // How to change Utc to Local
    // https://stackoverflow.com/questions/28747694/how-do-i-convert-a-chrono-datetimeutc-instance-to-datetimelocal
    // TODO:
    // add a option to see all opened slots
    // TODO:
    // show only booked slots.
    async fn print_slots(&self) -> Result<(), CliError> {
        let slots = self.get_slots().await?;
        let local = Local::now() + Duration::minutes(30);
        // let n: NaiveTime  = local.time().overflowing_add_signed(Duration::minutes(30)).0;
        for slot in slots.iter().rev() {
            let begin = slot.begin_at().parse::<DateTime<Utc>>()?;
            let end = slot.end_at().parse::<DateTime<Utc>>()?;
            let begin_diff = begin.with_timezone(&Local);
            let end_diff = end.with_timezone(&Local);
            if begin_diff > local {
                println!("Begin at: {}", begin_diff);
                println!("End at: {}", end_diff);
            }
        }
        Ok(())
    }
}

impl Program {
    async fn me(&self) -> Result<(), CliError> {
        let me = self.get_user().await?;
        me.me(self.config()).await?;
        Ok(())
    }

    async fn email(&self) -> Result<(), CliError> {
        let me = self.get_user().await?;
        me.email(self._config._detail, self._config._human);
        Ok(())
    }

    async fn projects(&self) -> Result<(), CliError> {
        let me = self.get_user().await?;
        me.projects(self._config._detail, self._config._human);
        Ok(())
    }

    async fn event(&self) -> Result<(), CliError> {
        let user = self.get_user().await?;
        let campus_id = user.campus[0].id;
        let url = format!("https://api.intra.42.fr/v2/campus/{}/events", campus_id);
        let url = Url::parse_with_params(&url, &[("client_id", self._client.client_id())])?;
        let res = session::call(
            self._client.access_token(),
            self._client.client_id(),
            url.as_str(),
        )
        .await?;
        let events: campus_event::CampusEvent = serde_json::from_str(res.as_str())?;

        let local = Local::now();
        for (_, event) in events.iter().rev().enumerate() {
            let begin = event.begin_at.parse::<DateTime<Local>>()?;
            let end = event.end_at.parse::<DateTime<Local>>()?;
            if end.signed_duration_since(local).num_seconds() > 0 {
                println!("🌈 🌈 🌈 {} 🌈 🌈 🌈\n", event.name);
                println!("⏰{:24}{}", "Begin at", begin);
                println!("⏰{:24}{}\n", "End at", end);
                if self._config._detail {
                    println!("{}\n", event.description);
                }
            }
        }
        Ok(())
    }
}
