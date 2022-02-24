use crate::client::Client;
use crate::cli::Cli;
use crate::error::CliError;
use crate::results::me::Me;
use crate::session;

use url::Url;

pub struct Program {
	_client: Client,
	_config: Cli,
}

impl Program {
	pub fn new(_client: Client, _config: Cli) -> Self {
		Self {
			_client,
			_config,
		}
	}

	pub fn config(&self) -> &Cli {
		&self._config
	}

	async fn get_me(&self) -> Result<Me, CliError> {
		let uri = "https://api.intra.42.fr/v2/me";
		let uri = Url::parse_with_params(uri, &[("client_id", self._client.client_id())])?;
		let res = session::call(
			self._client.access_token(),
			self._client.client_id(),
			uri.as_str()
		).await?;
		Ok(serde_json::from_str(res.as_str())?)
	}

	pub async fn me(&self) -> Result<(), CliError> {
		let me = self.get_me().await?;
		me.me(self.config()).await?;
		Ok(())
	}
}