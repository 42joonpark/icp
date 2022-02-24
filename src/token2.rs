use directories::BaseDirs;
use std::fs;

pub struct Client {
	client_id: String,
	client_secret: String,
	access_token: Option<String>,
	refresh_token: Option<String>,
}

impl Client {
	pub fn new() -> Self {
		let dir = BaseDirs::new().ok_or(CliError::BaseDirsNewError)?;
		let path = dir.config_dir().join("config.toml");
		let content = fs::read_to_string(path)?;
		let client = toml::from_str(&content)?;
		// if access token is None then generate one
		if client.access_token.is_none() {
			
		}
	}

	pub fn client_id(&self) -> &str {
		self.client_id.as_str()
	}
	pub fn client_secret(&self) -> &str {
		self.client_secret.as_str()
	}
	pub fn access_token(&self) -> Option<&str> {
		self.access_token.as_ref().map(|s| s.as_str())
	}
	pub fn refresh_token(&self) -> Option<&str> {
		self.refresh_token.as_ref().map(|s| s.as_str())
	}
}

impl Client {
	async fn generate_token(&self) -> Result<String, CliError> {
		let client = BasicClient::new(
			ClientId::new(String::from(session.client_id())),
			Some(ClientSecret::new(String::from(session.client_secret()))),
			AuthUrl::new("https://api.intra.42.fr/oauth/authorize".to_string())?,
			Some(TokenUrl::new(
				"https://api.intra.42.fr/oauth/token".to_string(),
			)?),
		)
		.set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string())?);
	
		let (auth_url, _) = client
			.authorize_url(CsrfToken::new_random)
			.add_scope(Scope::new("public".to_string()))
			.url();
		println!("Browse to: {}", auth_url);
	
		let ac_token = local_server(client).await?;
		Ok(ac_token)
	}
	
	async fn local_server(client: BasicClient) -> Result<String, CliError> {
		let ac_token;
		let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
		loop {
			if let Ok((mut stream, _)) = listener.accept().await {
				let code;
				let state;
				{
					let mut reader = BufReader::new(&mut stream);
					let mut request_line = String::new();
					reader.read_line(&mut request_line).await?;
					let redirect_url = match request_line.split_whitespace().nth(1) {
						Some(url) => url,
						None => return Err(CliError::IcpError("Failed to get redirect url.".into())),
					};
					let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;
	
					let code_pair = match url.query_pairs().find(|pair| {
						let &(ref key, _) = pair;
						key == "code"
					}) {
						Some(code) => code,
						None => return Err(CliError::IcpError("Failed to get code.".into())),
					};
	
					let (_, value) = code_pair;
					code = AuthorizationCode::new(value.into_owned());
	
					let state_pair = match url.query_pairs().find(|pair| {
						let &(ref key, _) = pair;
						key == "state"
					}) {
						Some(state) => state,
						None => return Err(CliError::IcpError("Failed to get state.".into())),
					};
	
					let (_, value) = state_pair;
					state = CsrfToken::new(value.into_owned());
				}
				let message = "Go back to your terminal :)";
				let response = format!(
					"HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
					message.len(),
					message
				);
				stream.write_all(response.as_bytes()).await?;
	
				debug!("42API returned the following code:\n{}\n", code.secret());
				debug!("42API returned the following state:\n{}\n", state.secret());
	
				let token_res = client
					.exchange_code(code)
					.request_async(async_http_client)
					.await;
				let token = match token_res {
					Err(_) => return Err(CliError::AuthError(AuthError::UnauthResponse)),
					Ok(t) => t,
				};
				debug!("42API returned the following token:\n{:?}\n", token);
	
				let scopes = if let Some(scopes_vec) = token.scopes() {
					scopes_vec
						.iter()
						.map(|comma_separated| comma_separated.split(','))
						.flatten()
						.collect::<Vec<_>>()
				} else {
					Vec::new()
				};
				ac_token = token.access_token().secret().to_owned();
				debug!("Access Token: {:?}", ac_token);
				debug!("42API returned the following scopes:\n{:?}\n", scopes);
				break;
			}
		}
		Ok(ac_token)
	}
}