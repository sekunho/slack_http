use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct BasicClient(pub(crate) reqwest::Client);

#[derive(Clone, Debug)]
pub struct AuthClient(pub(crate) reqwest::Client);

#[derive(Debug, Error)]
pub enum CreateClientError {
    #[error("failed to parse header value")]
    HeaderValue(#[from] InvalidHeaderValue),
    #[error("failed to build client")]
    Client(#[from] reqwest::Error),
}

impl AuthClient {
    pub fn new(token: String) -> Result<Self, CreateClientError> {
        let mut headers = HeaderMap::new();
        let bearer_token: HeaderValue = format!("Bearer {}", token).parse()?;

        headers.insert("Authorization", bearer_token);

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self(client))
    }
}

impl BasicClient {
    pub fn new() -> Result<BasicClient, CreateClientError> {
        let client = reqwest::Client::builder().build()?;

        Ok(BasicClient(client))
    }
}
