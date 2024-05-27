use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error<SlackError> {
    #[error("slack failed to process request. reason: {0}")]
    Slack(SlackError),
    #[error("failed to send request to slack. reason: {0}")]
    Request(reqwest::Error),
    #[error("failed to deserialize slack response. reason: {0}")]
    Deserialize(reqwest::Error),
    #[error("failed to parse URL. reason: {0}")]
    Url(#[from] url::ParseError),
}

impl<SlackError> Error<SlackError> {
    pub fn get_slack_error(&self) -> Option<&SlackError> {
        match self {
            Error::Slack(e) => Some(e),
            _ => None,
        }
    }
}
