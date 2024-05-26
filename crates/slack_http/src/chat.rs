use std::num::ParseIntError;

use serde::{de, Deserialize, Deserializer};
use time::OffsetDateTime;

use crate::client::AuthClient;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum MessageResponse {
    Ok {
        ok: bool,
        channel: String,
        message: Message,
        ts: String,
    },
    Error {
        ok: bool,
        error: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EphemeralMessageResponse {
    Ok {
        #[serde(rename = "message_ts")]
        #[serde(deserialize_with = "slack_http_types::offset_date_time_from_unix_ts_with_nano")]
        timestamp: OffsetDateTime,
    },
    Error {
        error: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub bot_id: String,
    pub text: String,
    pub user: String,
    pub app_id: String,
    #[serde(rename = "ts")]
    #[serde(deserialize_with = "slack_http_types::offset_date_time_from_unix_ts_with_nano")]
    pub timestamp: OffsetDateTime,
}

#[derive(Debug)]
pub enum PostMessageError {
    Slack(String),
    Request(reqwest::Error),
    Deserialize(reqwest::Error),
}

pub async fn post_message(
    client: &AuthClient,
    channel_id: &str,
    message: &str,
) -> Result<Message, PostMessageError> {
    let url = format!(
        "https://slack.com/api/chat.postMessage?channel={}&text={}",
        channel_id, message
    );
    let res = client
        .0
        .post(url)
        .send()
        .await
        .map_err(PostMessageError::Request)?;

    let json = res
        .json::<MessageResponse>()
        .await
        .map_err(PostMessageError::Deserialize)?;

    match json {
        MessageResponse::Ok { message, .. } => Ok(message),
        MessageResponse::Error { error, .. } => Err(PostMessageError::Slack(error)),
    }
}

pub async fn post_ephemeral(
    client: &AuthClient,
    channel_id: &str,
    user_id: &str,
    message: &str,
) -> Result<(), PostMessageError> {
    let url = format!(
        "https://slack.com/api/chat.postEphemeral?channel={}&user={}&text={}",
        channel_id, user_id, message
    );
    tracing::debug!("Requesting {}", url);
    let res = client
        .0
        .post(url)
        .send()
        .await
        .map_err(PostMessageError::Request)?;

    let json = res
        .json::<EphemeralMessageResponse>()
        .await
        .map_err(PostMessageError::Deserialize)?;

    match json {
        EphemeralMessageResponse::Ok { .. } => Ok(()),
        EphemeralMessageResponse::Error { error, .. } => Err(PostMessageError::Slack(error)),
    }
}
