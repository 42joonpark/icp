use crate::cli::Cli;
use crate::client::Client;
use crate::error::CliError;
use crate::results::campus_event;
use crate::results::me::Me;
use crate::session;

use chrono::{DateTime, Local};
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
            _ => println!("{}", format!("{} is not a valid command", command)),
        }
        Ok(())
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
}

impl Program {
    async fn me(&self) -> Result<(), CliError> {
        let me = self.get_me().await?;
        me.me(self.config()).await?;
        Ok(())
    }

    async fn email(&self) -> Result<(), CliError> {
        let me = self.get_me().await?;
        me.email();
        Ok(())
    }

    async fn event(&self) -> Result<(), CliError> {
        let user = self.get_me().await?;
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
