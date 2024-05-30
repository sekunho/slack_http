use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);
