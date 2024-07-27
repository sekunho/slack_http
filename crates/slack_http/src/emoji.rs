use std::collections::HashMap;

use crate::client::AuthClient;

pub use slack_http_types::emoji::ListResponse;
use slack_http_types::error::Error;
use url::Url;

const LIST: &str = "https://slack.com/api/emoji.list";

pub async fn list(auth_client: &AuthClient) -> Result<HashMap<String, String>, Error> {
    let url = Url::parse_with_params(LIST, &[("include_categories", "false")])?;

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
        ListResponse::Ok { emoji } => Ok(emoji),
        ListResponse::Error { error, .. } => Err(Error::Slack(error)),
    }
}
