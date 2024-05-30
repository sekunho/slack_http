use crate::client::{AuthClient, BasicClient};
use reqwest::Url;
use slack_http_types::{
    error::Error,
    oidc::{TokenResponse, UserInfoResponse},
};
use std::collections::HashMap;

pub use slack_http_types::oidc::{Token, UserInfo};

const POST_OIDC_TOKEN: &str = "https://slack.com/api/openid.connect.token";
const GET_USER_INFO: &str = "https://slack.com/api/openid.connect.userInfo";

pub async fn token(
    BasicClient(basic_client): &BasicClient,
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &Url,
) -> Result<Token, Error> {
    let mut params = HashMap::new();

    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("code", code);
    params.insert("redirect_uri", redirect_uri.as_str());

    let url = Url::parse(POST_OIDC_TOKEN)?;

    let res = basic_client
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(Error::Request)?;

    tracing::info!("POST {} -> {}", POST_OIDC_TOKEN, res.status());

    let json = res
        .json::<TokenResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        TokenResponse::Ok { access_token } => Ok(access_token),
        TokenResponse::Error { error } => Err(Error::Slack(error)),
    }
}

pub async fn user_info(AuthClient(auth_client): &AuthClient) -> Result<UserInfo, Error> {
    let url = Url::parse(GET_USER_INFO).expect("not a URL lol");
    let res = auth_client.get(url).send().await.map_err(Error::Request)?;
    tracing::info!("GET {} -> {}", GET_USER_INFO, res.status());

    let json = res
        .json::<UserInfoResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        UserInfoResponse::Ok(user) => Ok(user),
        UserInfoResponse::Error { error } => Err(Error::Slack(error)),
    }
}
