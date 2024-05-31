use std::fmt::Display;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::page::ResponseMetadata;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Id(pub String);

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: Id,
    pub deleted: bool,
    pub is_bot: bool,
    pub is_app_user: bool,
    pub is_admin: bool,
    pub is_owner: bool,
    pub is_restricted: bool,
    pub is_ultra_restricted: bool,
    pub profile: Profile,
    pub tz: String,
    pub tz_label: String,
    pub tz_offset: i64,
}

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub image_24: Url,
    pub image_32: Url,
    pub image_48: Url,
    pub image_72: Url,
    pub image_192: Url,
    pub image_512: Url,
    pub avatar_hash: String,
    pub status_text: String,
    pub status_emoji: String,
    pub display_name: String,
    pub display_name_normalized: String,
    pub real_name: String,
    pub real_name_normalized: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ListResponse {
    Ok {
        members: Vec<User>,
        response_metadata: ResponseMetadata,
    },
    Error {
        error: String,
    },
}

impl Id {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
