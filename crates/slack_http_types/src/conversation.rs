use serde::Deserialize;
use time::OffsetDateTime;

use crate::{offset_date_time_from_unix_ts, option::Limit, user};

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(transparent)]
pub struct Id(pub String);

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Conversation {
    pub id: Id,
    pub name: String,
    pub name_normalized: String,
    #[serde(deserialize_with = "offset_date_time_from_unix_ts")]
    pub created: OffsetDateTime,
    pub creator: user::Id,
    pub is_member: bool,
    pub is_archived: bool,
    pub is_private: bool,
    pub is_channel: bool,
    pub is_group: bool,
    pub is_im: bool,
    pub is_mpim: bool,
    pub is_general: bool,
}

// LIST CHANNELS
#[derive(Debug, Deserialize)]
pub struct ListResponseMetadata {
    pub next_cursor: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ListResponse {
    Ok {
        channels: Vec<Conversation>,
        response_metadata: ListResponseMetadata,
    },
    Error {
        error: String,
    },
}

#[derive(Copy, Clone)]
pub struct ListOptions {
    pub limit: Limit,
    pub exclude_archived: bool,
    pub include_public: bool,
    pub include_private: bool,
    pub include_mpim: bool,
    pub include_im: bool,
}

impl Id {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            limit: Default::default(),
            exclude_archived: false,
            include_public: true,
            include_private: false,
            include_mpim: false,
            include_im: false,
        }
    }
}

impl ListOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_limit(self, limit: Limit) -> Self {
        Self { limit, ..self }
    }

    pub fn include_public(self, include: bool) -> Self {
        Self {
            include_public: include,
            ..self
        }
    }

    pub fn include_private(self, include: bool) -> Self {
        Self {
            include_private: include,
            ..self
        }
    }

    pub fn include_mpim(self, include: bool) -> Self {
        Self {
            include_mpim: include,
            ..self
        }
    }

    pub fn include_im(self, include: bool) -> Self {
        Self {
            include_im: include,
            ..self
        }
    }

    pub fn types_query_param(&self) -> String {
        let mut types = Vec::new();

        if self.include_public {
            types.push("public_channel");
        }

        if self.include_private {
            types.push("private_channel")
        }

        if self.include_mpim {
            types.push("mpim")
        }

        if self.include_im {
            types.push("im")
        }

        types.join(",")
    }
}

// INVITE TO RESPONSES
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InviteResponse {
    Ok { channel: Conversation },
    Error { error: String },
}

// KICK FROM CHANNEL
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum KickResponse{
    Ok { ok: bool },
    Error { error: String },
}
