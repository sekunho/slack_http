use std::collections::HashMap;

use crate::client::AuthClient;
use reqwest::Url;
use serde::Deserialize;
use thiserror::Error;

pub use slack_http_types::team::Id;

const GET_TEAM_INFO: &str = "https://slack.com/api/team.info";

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: Id,
    pub name: String,
    pub domain: String,
    pub icon: Icon,
    // pub enterprise_id: String,
    // pub enterprise_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Icon {
    pub image_132: Url,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum GetTeamInfoResponse {
    Ok { team: Team },
    Error { error: SlackGetTeamInfoError },
}

#[derive(Debug, Error)]
pub enum GetTeamInfoError {
    #[error("failed when requesting slack to process request")]
    Request(reqwest::Error),
    #[error("failed to deserialize slack response")]
    Deserialize(reqwest::Error),
    #[error("slack failed to process request")]
    Slack(SlackGetTeamInfoError),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlackGetTeamInfoError {}

pub async fn get_team_info(
    auth_client: &AuthClient,
    team_id: Id,
) -> Result<Team, GetTeamInfoError> {
    let mut params: HashMap<String, String> = HashMap::new();
    let url = Url::parse(GET_TEAM_INFO).expect("not a URL lol");

    params.insert(String::from("team"), team_id.0);

    let res = auth_client
        .client()
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(GetTeamInfoError::Request)?;

    let json = res
        .json::<GetTeamInfoResponse>()
        .await
        .map_err(GetTeamInfoError::Deserialize)?;

    match json {
        GetTeamInfoResponse::Ok { team } => Ok(team),
        GetTeamInfoResponse::Error { error } => {
            // TODO: print out reason
            tracing::error!("failed to get team info from Slack");
            Err(GetTeamInfoError::Slack(error))
        }
    }
}
