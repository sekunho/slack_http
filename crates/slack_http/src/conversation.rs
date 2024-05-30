use slack_http_types::{
    conversation::{InviteResponse, KickResponse, MembersResponse, OpenResponse},
    error::Error,
    page::{Cursor, Limit, Page},
    user,
};
use url::Url;

use crate::client::AuthClient;
pub use slack_http_types::conversation::{Conversation, Id, ListOptions};

pub async fn members(
    auth_client: &AuthClient,
    conversation_id: &Id,
    cursor: &Cursor,
    limit: Limit,
) -> Result<Page<user::Id>, Error> {
    let url = Url::parse_with_params(
        "https://slack.com/api/conversations.members",
        &[
            ("channel", conversation_id.as_str()),
            ("cursor", cursor.as_str()),
            ("limit", limit.get().to_string().as_str()),
        ],
    )?;
    let res = auth_client
        .client()
        .post(url)
        .send()
        .await
        .map_err(Error::Request)?;

    let json = res
        .json::<MembersResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        MembersResponse::Ok {
            members,
            response_metadata,
        } => Ok(Page::new(members, Cursor::from(response_metadata))),
        MembersResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}

pub async fn open(auth_client: &AuthClient, user_ids: Vec<user::Id>) -> Result<Id, Error> {
    let url = Url::parse_with_params(
        "https://slack.com/api/conversations.open",
        &[(
            "users",
            user_ids
                .into_iter()
                .map(|uid| uid.0)
                .collect::<Vec<String>>()
                .join(","),
        )],
    )?;
    let res = auth_client
        .client()
        .post(url)
        .send()
        .await
        .map_err(Error::Request)?;

    let json = res
        .json::<OpenResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        OpenResponse::Ok { channel, .. } => Ok(channel.id),
        OpenResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}

pub async fn invite<'channel_id>(
    auth_client: &AuthClient,
    channel_id: &slack_http_types::conversation::Id,
    user_ids: Vec<slack_http_types::user::Id>,
) -> Result<(), Error> {
    let url = Url::parse_with_params(
        "https://slack.com/api/conversations.invite",
        &[
            ("channel", channel_id.as_str()),
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

    let res = auth_client
        .client()
        .post(url.as_str())
        .send()
        .await
        .map_err(Error::Request)?;

    let status = res.status();

    tracing::info!("POST {} -> {}", url, status);

    let json = res
        .json::<slack_http_types::conversation::InviteResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        InviteResponse::Ok { .. } => Ok(()),
        InviteResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}

pub async fn kick(
    auth_client: &AuthClient,
    conversation_id: &slack_http_types::conversation::Id,
    user_id: &slack_http_types::user::Id,
) -> Result<(), Error> {
    let url = Url::parse_with_params(
        "https://slack.com/api/conversations.kick",
        &[
            ("channel", conversation_id.as_str()),
            ("user", user_id.as_str()),
        ],
    )?;

    let res = auth_client
        .client()
        .post(url.as_str())
        .send()
        .await
        .map_err(Error::Request)?;

    let status = res.status();

    tracing::info!("POST {} -> {}", url, status);

    let json = res
        .json::<KickResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        KickResponse::Ok { .. } => Ok(()),
        KickResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}

/// Lists channels/mpim/im in the Slack workspace
pub async fn list(
    client: &AuthClient,
    team_id: &str,
    cursor: &Cursor,
    params: slack_http_types::conversation::ListOptions,
) -> Result<Page<Conversation>, Error> {
    let url = Url::parse_with_params(
        "https://slack.com/api/conversations.list",
        &[
            ("cursor", cursor.as_str()),
            ("types", params.types_query_param().as_str()),
            ("limit", params.limit.get().to_string().as_str()),
            ("team_id", team_id),
        ],
    )?;

    tracing::info!("POST {}", url.to_string());

    let res = client
        .client()
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
        } => Ok(Page::new(channels, Cursor::from(response_metadata))),
        slack_http_types::conversation::ListResponse::Error { error, .. } => {
            Err(Error::Slack(error))
        }
    }
}
