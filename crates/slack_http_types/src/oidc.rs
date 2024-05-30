use serde::Deserialize;
use url::Url;

use crate::team;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct Token(pub String);

/// Slack's temporary OAuth2 verifier code. Exchange this with an access token,
/// and refresh token (if token rotation is enabled).
pub struct Code(pub String);

#[derive(Deserialize)]
#[serde(untagged)]
pub enum TokenResponse {
    Ok { access_token: Token },
    Error { error: String },
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "https://slack.com/user_id")]
    pub id: String,
    #[serde(rename = "https://slack.com/team_id")]
    pub team_id: team::Id,
    pub picture: Url,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum UserInfoResponse {
    Ok(UserInfo),
    Error { error: String },
}
