use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

use crate::client::BasicClient;

const V2_ACCESS: &str = "https://slack.com/api/oauth.v2.access";

#[derive(Debug, Error)]
pub enum OAuthV2AccessError {}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserToken(String);

#[derive(Debug, Deserialize, Serialize)]
pub struct BotToken(String);

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRefreshToken(String);

#[derive(Debug, Deserialize, Serialize)]
pub struct BotRefreshToken(String);

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum OAuthV2AccessResponse {
    Ok(Box<Access>),
    Error { error: String },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum OAuthV2RefreshResponse {
    Ok(RefreshedAccess),
    Error { error: String },
}

#[derive(Deserialize, Debug)]
pub enum TokenType {
    #[serde(rename = "bot")]
    Bot,
    #[serde(rename = "user")]
    User,
}

#[derive(Debug, Deserialize)]
pub struct RefreshedAccess {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: TokenType,
    pub scope: String,
    pub app_id: String,
    pub team: Team,
}

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: crate::team::Id,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthedUser {
    pub id: String,
    pub scope: String,
    pub access_token: UserToken,
    pub expires_in: u64,
    pub refresh_token: UserRefreshToken,
}

#[derive(Debug, Deserialize)]
pub struct Access {
    #[serde(rename = "access_token")]
    pub bot_access_token: BotToken,
    #[serde(rename = "refresh_token")]
    pub bot_refresh_token: BotRefreshToken,
    pub expires_in: u64,
    pub bot_user_id: String,
    pub app_id: String,
    pub scope: String,
    pub team: Team,
    pub authed_user: AuthedUser,
}

////////////////////////////////////////////////////////////////////////////////
/// Type impls

impl UserToken {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl BotToken {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl UserRefreshToken {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl BotRefreshToken {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Functions

#[derive(Debug, Error)]
pub enum GetAccessTokenError {
    #[error("slack failed to provide the access token {0}")]
    Slack(String),
    #[error("failed to send request to slack {0}")]
    Request(reqwest::Error),
    #[error("failed to deserialize slack response {0}")]
    Deserialize(reqwest::Error),
}

pub async fn v2_refresh_access(
    basic_client: &BasicClient,
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<RefreshedAccess, GetAccessTokenError> {
    tracing::info!("POST https://slack.com/api/oauth.v2.access");
    let mut params = HashMap::new();

    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("refresh_token", refresh_token);
    params.insert("grant_type", "refresh_token");

    let url = Url::parse(V2_ACCESS).expect("not a URL");

    let res = basic_client
        .0
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(GetAccessTokenError::Request)?;

    let json = res
        .json::<OAuthV2RefreshResponse>()
        .await
        .map_err(GetAccessTokenError::Deserialize)?;

    tracing::debug!("Access details {:#?}", json);

    match json {
        OAuthV2RefreshResponse::Ok(access) => Ok(access),
        OAuthV2RefreshResponse::Error { error, .. } => Err(GetAccessTokenError::Slack(error)),
    }
}

// TODO: Maybe distinguish basic client and bearer
pub async fn v2_access(
    basic_client: &BasicClient,
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<Access, GetAccessTokenError> {
    let mut params = HashMap::new();

    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("code", code);
    params.insert("redirect_uri", redirect_uri);

    let url = Url::parse(V2_ACCESS).expect("not a URL");

    let res = basic_client
        .0
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(GetAccessTokenError::Request)?;

    let json = res
        .json::<OAuthV2AccessResponse>()
        .await
        .map_err(GetAccessTokenError::Deserialize)?;

    tracing::debug!("Access details {:#?}", json);

    match json {
        OAuthV2AccessResponse::Ok(access) => Ok(*access),
        OAuthV2AccessResponse::Error { error, .. } => Err(GetAccessTokenError::Slack(error)),
    }
}
