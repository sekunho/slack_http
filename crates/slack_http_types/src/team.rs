use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Id(pub String);

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

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InfoResponse {
    Ok { team: Team },
    Error { error: String },
}
