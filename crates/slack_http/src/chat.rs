use slack_http_types::{
    chat::{EphemeralResponse, MessageResponse},
    conversation,
    error::Error,
    user,
};
use time::OffsetDateTime;
use url::Url;

use crate::client::AuthClient;
pub use slack_http_types::chat::{Message, MessageOptions};

pub async fn post_message(
    auth_client: &AuthClient,
    conversation_id: &conversation::Id,
    message: &str,
    opts: &MessageOptions,
) -> Result<Message, Error> {
    let mut query_params = opts.query_params();

    query_params.push(("channel", conversation_id.as_str()));
    query_params.push(("text", message));

    let url = Url::parse_with_params("https://slack.com/api/chat.postMessage", &query_params)?;
    let res = auth_client
        .client()
        .post(url)
        .send()
        .await
        .map_err(Error::Request)?;

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
    auth_client: &AuthClient,
    conversation_id: &conversation::Id,
    user_id: &user::Id,
    message: &str,
    opts: &MessageOptions,
) -> Result<OffsetDateTime, Error> {
    let mut query_params = opts.query_params();

    query_params.push(("channel", conversation_id.as_str()));
    query_params.push(("user", user_id.as_str()));
    query_params.push(("text", message));

    let url = Url::parse_with_params("https://slack.com/api/chat.postEphemeral", &query_params)?;
    tracing::debug!("Requesting {}", url);
    let res = auth_client
        .client()
        .post(url)
        .send()
        .await
        .map_err(Error::Request)?;

    let json = res
        .json::<EphemeralResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        EphemeralResponse::Ok { timestamp } => Ok(timestamp),
        EphemeralResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}
