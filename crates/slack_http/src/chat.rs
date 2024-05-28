use serde::Deserialize;
use slack_http_types::{chat::{Message, MessageOptions}, conversation, error::Error};
use time::OffsetDateTime;
use url::Url;

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

pub async fn post_message(
    client: &AuthClient,
    conversation_id: &conversation::Id,
    message: &str,
    opts: &MessageOptions,
) -> Result<Message, Error<String>> {
    let url = Url::parse_with_params(
        "https://slack.com/api/chat.postMessage",
        &[
            ("channel", conversation_id.as_str()),
            ("text", message),
        ],
    )?;
    let res = client.0.post(url).send().await.map_err(Error::Request)?;

    let json = res
        .json::<MessageResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        MessageResponse::Ok { message, .. } => Ok(message),
        MessageResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}

pub async fn post_ephemeral(
    client: &AuthClient,
    channel_id: &str,
    user_id: &str,
    message: &str,
) -> Result<(), Error<String>> {
    let url = format!(
        "https://slack.com/api/chat.postEphemeral?channel={}&user={}&text={}",
        channel_id, user_id, message
    );
    tracing::debug!("Requesting {}", url);
    let res = client.0.post(url).send().await.map_err(Error::Request)?;

    let json = res
        .json::<EphemeralMessageResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        EphemeralMessageResponse::Ok { .. } => Ok(()),
        EphemeralMessageResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}
