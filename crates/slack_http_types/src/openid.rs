use serde::Deserialize;
use url::Url;

use crate::{oauth::AccessToken, team, user};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum TokenResponse {
    Ok { access_token: AccessToken },
    Error { error: String },
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "https://slack.com/user_id")]
    pub id: user::Id,
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
