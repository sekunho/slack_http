use serde::Deserialize;
use url::Url;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
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
