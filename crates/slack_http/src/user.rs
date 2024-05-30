use reqwest::Url;
use serde::Deserialize;
use thiserror::Error;

use crate::client::AuthClient;

pub use slack_http_types::user::Id;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Cursor(String);

#[derive(Debug, Deserialize)]
struct User {
    id: Id,
    deleted: bool,
    is_bot: bool,
    profile: Profile,
}

#[derive(Debug, Deserialize)]
struct Profile {
    #[serde(rename = "image_512")]
    picture_url: Url,
    real_name: String,
}

/// A user who's still active and not a bot
#[derive(Debug, Clone)]
pub struct ActiveUser {
    pub id: Id,
    pub name: String,
    pub picture_url: Url,
}

#[derive(Debug, Error)]
pub enum ListActiveUsersError {
    #[error("failed to send request to Slack")]
    Request(reqwest::Error),
    #[error("failed to deserialize Slack response")]
    Deserialize(reqwest::Error),
    #[error("slack failed to process request")]
    Slack(String),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ListActiveUsersResponse {
    Ok {
        members: Vec<User>,
        response_metadata: ResponseMetadata,
    },
    // TODO: Enum
    Error {
        error: String,
    },
}

#[derive(Debug, Deserialize)]
struct ResponseMetadata {
    next_cursor: Cursor,
}

#[derive(Debug, Error)]
pub enum ParseActiveUserError {
    #[error("cannot parse a deleted user into an active user {0}")]
    Deleted(Id),
    #[error("cannot parse a bot user into an active user {0}")]
    Bot(Id),
}

impl TryFrom<User> for ActiveUser {
    type Error = ParseActiveUserError;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        match user {
            User {
                id, deleted: true, ..
            } => Err(ParseActiveUserError::Deleted(id)),
            User {
                id, is_bot: true, ..
            } => Err(ParseActiveUserError::Bot(id)),
            user => Ok(ActiveUser {
                id: user.id,
                name: user.profile.real_name,
                picture_url: user.profile.picture_url,
            }),
        }
    }
}

pub async fn list_active_users(
    auth_client: &AuthClient,
    team_id: &crate::team::Id,
    cursor: Option<Cursor>,
) -> Result<(Vec<ActiveUser>, Option<Cursor>), ListActiveUsersError> {
    let url = format!(
        "https://slack.com/api/users.list?cursor={}&limit=200&team_id={}",
        // Wow, it's so ugly!
        cursor.unwrap_or(Cursor(String::from(""))).0,
        team_id.0
    );

    let res = auth_client
        .client()
        .post(url)
        .send()
        .await
        .map_err(ListActiveUsersError::Request)?;

    let json = res
        .json::<ListActiveUsersResponse>()
        .await
        .map_err(ListActiveUsersError::Deserialize)?;

    match json {
        ListActiveUsersResponse::Ok {
            members,
            response_metadata,
            ..
        } => {
            let cursor = if response_metadata.next_cursor.0.as_str() == "" {
                None
            } else {
                Some(response_metadata.next_cursor)
            };

            let active_members = members
                .into_iter()
                .map(ActiveUser::try_from)
                .filter_map(|rau| rau.ok())
                .collect::<Vec<ActiveUser>>();

            Ok((active_members, cursor))
        }
        ListActiveUsersResponse::Error { error, .. } => Err(ListActiveUsersError::Slack(error)),
    }
}
