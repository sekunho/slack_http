use crate::client::AuthClient;
use reqwest::Url;
use serde::Deserialize;
use slack_http_types::{error::Error, team::InfoResponse};

pub use slack_http_types::team::{Id, Team};

const GET_TEAM_INFO: &str = "https://slack.com/api/team.info";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackError {}

pub async fn info(auth_client: &AuthClient, team_id: &Id) -> Result<Team, Error> {
    let url = Url::parse_with_params(GET_TEAM_INFO, &[("team", team_id.0.as_str())])?;

    let res = auth_client
        .client()
        .get(url)
        .send()
        .await
        .map_err(Error::Request)?;

    tracing::info!("GET {} -> {}", GET_TEAM_INFO, res.status());

    let json = res
        .json::<InfoResponse>()
        .await
        .map_err(Error::Deserialize)?;

    match json {
        InfoResponse::Ok { team } => Ok(team),
        InfoResponse::Error { error } => {
            tracing::error!("failed to get team info from Slack");
            Err(Error::Slack(error))
        }
    }
}
