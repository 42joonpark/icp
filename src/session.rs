use crate::error::{AuthError, TokenError};
use crate::CliError;
use log::{self, debug, warn};
use reqwest::header::AUTHORIZATION;

pub async fn call(
    access_token: Option<&str>,
    client_id: &str,
    uri: &str,
) -> Result<String, CliError> {
    let ac_token = access_token.ok_or(CliError::TokenError(TokenError::NoAccessToken))?;
    let client = reqwest::Client::new();
    let params = [("client_id", client_id)];
    let response = client
        .get(uri.to_string())
        .header(AUTHORIZATION, format!("Bearer {}", ac_token))
        .form(&params)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            debug!("icp::Session::call(): reqwest OK");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            warn!("icp::Session::call(): unauthorized");
            // return Err(CliError::UnauthorizedResponse);
            return Err(CliError::AuthError(AuthError::UnauthResponse));
        }
        reqwest::StatusCode::FORBIDDEN => {
            warn!("icp::Session::call(): 402 FORBIDDEN ACCESS");
            return Err(CliError::AuthError(AuthError::Forbidden));
        }
        reqwest::StatusCode::NOT_FOUND => {
            warn!("icp::Session::call(): 404 NOT FOUND");
            return Err(CliError::AuthError(AuthError::NotFound));
        }
        _ => {
            panic!("uh oh! something unexpected happened");
        }
    }
    let res = response
        .text()
        .await
        .map_err(|_| CliError::IcpError("Failed to map GET response to text.".to_string()))?;
    Ok(res)
}
