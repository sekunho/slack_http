use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use thiserror::Error;

use crate::oauth::OauthToken;

#[derive(Clone, Debug)]
pub struct BasicClient(pub(crate) reqwest::Client);

#[derive(Clone, Debug)]
pub struct AuthClient(pub(crate) reqwest::Client);

#[derive(Debug, Error)]
pub enum CreateClientError {
    #[error("failed to parse header value")]
    HeaderValue(#[from] InvalidHeaderValue),
    #[error("failed to build client. reason: {0}")]
    Client(#[from] reqwest::Error),
}

impl AuthClient {
    pub fn new(token: OauthToken) -> Result<Self, CreateClientError> {
        let mut headers = HeaderMap::new();
        let bearer_token: HeaderValue = format!("Bearer {}", token.0).parse()?;
        headers.insert("Authorization", bearer_token);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self(client))
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.0
    }
}

impl BasicClient {
    pub fn new() -> Result<BasicClient, CreateClientError> {
        let client = reqwest::Client::builder().build()?;

        Ok(BasicClient(client))
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.0
    }
}
