use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("slack failed to process request. reason: {0}")]
    Slack(String),
    #[error("failed to send request to slack. reason: {0}")]
    Request(reqwest::Error),
    #[error("failed to deserialize slack response. reason: {0}")]
    Deserialize(reqwest::Error),
    #[error("failed to parse URL. reason: {0}")]
    Url(#[from] url::ParseError),
}

impl Error {
    pub fn get_slack_error(&self) -> Option<&str> {
        match self {
            Error::Slack(e) => Some(e.as_str()),
            _ => None,
        }
    }
}
