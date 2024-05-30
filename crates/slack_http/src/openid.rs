use crate::client::{AuthClient, BasicClient};
use reqwest::Url;
use slack_http_types::{
    error::Error,
    oauth::{AccessToken, Code},
    openid::{TokenResponse, UserInfoResponse},
};
use std::collections::HashMap;

pub use slack_http_types::openid::UserInfo;

const OIDC_TOKEN: &str = "https://slack.com/api/openid.connect.token";
const USER_INFO: &str = "https://slack.com/api/openid.connect.userInfo";

pub async fn token(
    basic_client: &BasicClient,
    client_id: &str,
    client_secret: &str,
    code: &Code,
    redirect_uri: &Url,
) -> Result<AccessToken, Error> {
    let mut params = HashMap::new();

    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("code", code.0.as_str());
    params.insert("redirect_uri", redirect_uri.as_str());

    let url = Url::parse(OIDC_TOKEN)?;

    let res = basic_client
        .client()
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(Error::Request)?;

    tracing::info!("POST {} -> {}", OIDC_TOKEN, res.status());

    let json = res
        .json::<TokenResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        TokenResponse::Ok { access_token } => Ok(access_token),
        TokenResponse::Error { error } => Err(Error::Slack(error)),
    }
}

pub async fn user_info(auth_client: &AuthClient) -> Result<UserInfo, Error> {
    let url = Url::parse(USER_INFO).expect("not a URL lol");

    let res = auth_client
        .client()
        .get(url)
        .send()
        .await
        .map_err(Error::Request)?;

    tracing::info!("GET {} -> {}", USER_INFO, res.status());

    let json = res
        .json::<UserInfoResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        UserInfoResponse::Ok(user) => Ok(user),
        UserInfoResponse::Error { error } => Err(Error::Slack(error)),
    }
}
