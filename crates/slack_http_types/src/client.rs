use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use thiserror::Error;

use crate::oauth::AccessToken;

// TODO: Set visibility back to pub(crate)
#[derive(Clone, Debug)]
pub struct BasicClient(pub reqwest::Client);

#[derive(Clone, Debug)]
pub struct AuthClient(pub reqwest::Client);

#[derive(Debug, Error)]
pub enum CreateClientError {
    #[error("failed to parse header value")]
    HeaderValue(#[from] InvalidHeaderValue),
    #[error("failed to build client. reason: {0}")]
    Client(#[from] reqwest::Error),
}

impl AuthClient {
    pub fn new(token: AccessToken) -> Result<Self, CreateClientError> {
        let mut headers = HeaderMap::new();
        let bearer_token: HeaderValue = format!("Bearer {}", token.0).parse()?;
        headers.insert("Authorization", bearer_token);

        let client = reqwest::Client::builder()
            .connection_verbose(true)
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
        let client = reqwest::Client::builder()
            .connection_verbose(true)
            .build()?;

        Ok(BasicClient(client))
    }

    pub fn client(&self) -> &reqwest::Client {
        &self.0
    }
}
