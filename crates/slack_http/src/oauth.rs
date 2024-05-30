use std::collections::HashMap;

use crate::client::BasicClient;
use url::Url;

use slack_http_types::{
    error::Error,
    oauth::{OAuthV2AccessResponse, OAuthV2RefreshResponse},
};

pub use slack_http_types::oauth::{Access, OauthToken, RefreshToken, RefreshedAccess, Team};

const V2_ACCESS: &str = "https://slack.com/api/oauth.v2.access";

////////////////////////////////////////////////////////////////////////////////
/// Functions

pub async fn v2_refresh_access(
    basic_client: &BasicClient,
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<RefreshedAccess, Error> {
    let mut params = HashMap::new();

    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("refresh_token", refresh_token);
    params.insert("grant_type", "refresh_token");

    let url = Url::parse(V2_ACCESS)?;

    let res = basic_client
        .client()
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(Error::Request)?;

    tracing::info!("POST {} -> {}", V2_ACCESS, res.status());

    let json = res
        .json::<OAuthV2RefreshResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        OAuthV2RefreshResponse::Ok(access) => Ok(access),
        OAuthV2RefreshResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}

pub async fn v2_access(
    basic_client: &BasicClient,
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<Access, Error> {
    let mut params = HashMap::new();

    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    params.insert("code", code);
    params.insert("redirect_uri", redirect_uri);

    let url = Url::parse(V2_ACCESS)?;

    let res = basic_client
        .client()
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(Error::Request)?;

    tracing::info!("POST {} -> {}", V2_ACCESS, res.status());

    let json = res
        .json::<OAuthV2AccessResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        OAuthV2AccessResponse::Ok(access) => Ok(*access),
        OAuthV2AccessResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}
