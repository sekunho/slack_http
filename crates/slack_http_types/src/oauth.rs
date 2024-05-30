use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct OauthToken(pub String);

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct RefreshToken(pub String);

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: crate::team::Id,
    pub name: String,
}

// ACCESS
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OAuthV2AccessResponse {
    Ok(Box<Access>),
    Error { error: String },
}

#[derive(Debug, Deserialize)]
pub struct Access {
    #[serde(rename = "access_token")]
    pub bot_access_token: OauthToken,
    #[serde(rename = "refresh_token")]
    pub bot_refresh_token: RefreshToken,
    pub expires_in: u64,
    pub bot_user_id: String,
    pub app_id: String,
    pub scope: String,
    pub team: Team,
    pub authed_user: AuthedUser,
}

#[derive(Debug, Deserialize)]
pub struct AuthedUser {
    pub id: String,
    pub scope: String,
    pub access_token: OauthToken,
    pub expires_in: u64,
    pub refresh_token: RefreshToken,
}

// REFRESH ACCESS
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum OAuthV2RefreshResponse {
    Ok(RefreshedAccess),
    Error { error: String },
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

#[derive(Deserialize, Debug)]
pub enum TokenType {
    #[serde(rename = "bot")]
    Bot,
    #[serde(rename = "user")]
    User,
}
