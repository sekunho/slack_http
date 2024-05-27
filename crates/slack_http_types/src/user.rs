use std::fmt::Display;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(transparent)]
pub struct Id(pub String);

impl Id {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Slack user ID: {}", self.0)
    }
}
