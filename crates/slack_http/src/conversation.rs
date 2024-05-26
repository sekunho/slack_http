use serde::Deserialize;
use thiserror::Error;
use url::Url;

use crate::{client::AuthClient, page::Page};
pub use slack_http_types::conversation::{Conversation, ListOptions};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum OpenConversationResponse {
    Ok { channel: DirectMessage },
    Error { error: String },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ConversationKickResponse {
    Ok { ok: bool },
    Error { error: String },
}

#[derive(Debug, Deserialize)]
pub struct DirectMessage {
    pub id: String,
}

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

pub async fn open(client: &AuthClient, user_id: &str) -> Result<DirectMessage, Error> {
    let url = format!("https://slack.com/api/conversations.open?users={}", user_id);
    let res = client.0.post(url).send().await.map_err(Error::Request)?;

    let json = res
        .json::<OpenConversationResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        OpenConversationResponse::Ok { channel, .. } => Ok(channel),
        OpenConversationResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}

pub async fn invite<'channel_id>(
    client: &AuthClient,
    channel_id: &slack_http_types::conversation::Id,
    user_ids: Vec<slack_http_types::user::Id>,
) -> Result<(), Error> {
    let url = Url::parse_with_params(
        "https://slack.com/api/conversations.invite",
        &[
            ("channel", channel_id.0.as_str()),
            (
                "users",
                user_ids
                    .into_iter()
                    .map(|uid| uid.0)
                    .collect::<Vec<String>>()
                    .join(",")
                    .as_str(),
            ),
        ],
    )?;

    let res = client
        .0
        .post(url.as_str())
        .send()
        .await
        .map_err(Error::Request)?;

    let status = res.status();

    tracing::info!("POST {} -> {}", url, status);

    let json = res
        .json::<slack_http_types::conversation::InviteResponse>()
        .await
        .map_err(|err| Error::Deserialize(err))?;

    match json {
        slack_http_types::conversation::InviteResponse::Ok { .. } => Ok(()),
        slack_http_types::conversation::InviteResponse::Error { error, .. } => {
            Err(Error::Slack(error))
        }
    }
}

pub async fn kick<'channel_id>(
    client: &AuthClient,
    channel_id: &'channel_id str,
    user_id: &str,
) -> Result<&'channel_id str, (&'channel_id str, Error)> {
    let url = format!(
        "https://slack.com/api/conversations.kick?channel={}&user={}",
        channel_id, user_id,
    );

    let res = client
        .0
        .post(url.as_str())
        .send()
        .await
        .map_err(|err| (channel_id, Error::Request(err)))?;

    let status = res.status();

    let json = res
        .json::<ConversationKickResponse>()
        .await
        .map_err(|err| (channel_id, Error::Deserialize(err)))?;

    tracing::info!("POST {} -> {}", url, status);

    match json {
        ConversationKickResponse::Ok { .. } => Ok(channel_id),
        ConversationKickResponse::Error { error, .. } => Err((channel_id, Error::Slack(error))),
    }
}

/// Lists channels/mpim/im in the Slack workspace
pub async fn list(
    client: &AuthClient,
    team_id: &str,
    cursor: Option<&str>,
    params: slack_http_types::conversation::ListOptions,
) -> Result<Page<Conversation>, Error> {
    let url = Url::parse_with_params(
        "https://slack.com/api/conversations.list",
        &[
            ("cursor", cursor.unwrap_or("")),
            ("types", params.types_query_param().as_str()),
            ("limit", &params.limit.get().to_string().as_str()),
            ("team_id", team_id),
        ],
    )?;

    tracing::info!("POST {}", url.to_string());

    let res = client
        .0
        .post(url.as_str())
        .send()
        .await
        .map_err(Error::Request)?;

    let json = res
        .json::<slack_http_types::conversation::ListResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        slack_http_types::conversation::ListResponse::Ok {
            channels,
            response_metadata,
            ..
        } => {
            let cursor = if response_metadata.next_cursor.as_str() == "" {
                None
            } else {
                Some(response_metadata.next_cursor)
            };

            Ok(Page::new(channels, cursor))
        }
        slack_http_types::conversation::ListResponse::Error { error, .. } => {
            Err(Error::Slack(error))
        }
    }
}
