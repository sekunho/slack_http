use crate::client::AuthClient;
pub use slack_http_types::user::Id;
use slack_http_types::{
    error::Error,
    page::{Cursor, Limit, Page},
    user::{ListResponse, User},
};
use url::Url;

const LIST: &str = "https://slack.com/api/users.list";

pub async fn list(
    auth_client: &AuthClient,
    team_id: Option<&crate::team::Id>,
    cursor: &Cursor,
    limit: &Limit,
) -> Result<Page<User>, Error> {
    let limit = limit.get().to_string();

    let url = Url::parse_with_params(
        LIST,
        &[
            ("team_id", team_id.map(|tid| tid.0.as_str()).unwrap_or("")),
            ("limit", limit.as_str()),
            ("cursor", cursor.as_str()),
            ("include_locale", "true"),
        ],
    )?;

    let res = auth_client
        .client()
        .get(url)
        .send()
        .await
        .map_err(Error::Request)?;

    let json = res
        .json::<ListResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        ListResponse::Ok {
            members,
            response_metadata,
            ..
        } => Ok(Page::new(members, Cursor::from(response_metadata))),
        ListResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}
