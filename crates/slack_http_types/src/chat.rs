use serde::Deserialize;
use time::OffsetDateTime;
use url::Url;

// POST MESSAGE
// TODO: Add the following options:
// 1. thread_ts
// 2. reply_broadcast
// 3. parse
// 4. metadata
pub struct MessageOptions {
    pub icon_emoji: Option<String>,
    pub icon_url: Option<Url>,
    pub link_names: bool,
    pub markdown: bool,
    pub unfurl_links: Option<bool>,
    pub unfurl_media: Option<bool>,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub bot_id: String,
    pub text: String,
    pub user: String,
    pub app_id: String,
    #[serde(rename = "ts")]
    #[serde(deserialize_with = "crate::offset_date_time_from_unix_ts_with_nano")]
    pub timestamp: OffsetDateTime,
}

impl MessageOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for MessageOptions {
    fn default() -> Self {
        Self {
            icon_emoji: None,
            icon_url: None,
            link_names: true,
            markdown: true,
            unfurl_links: None,
            unfurl_media: None,
            username: None,
        }
    }
}

impl Into<Vec<(String, String)>> for MessageOptions {
    fn into(self) -> Vec<(String, String)> {
        vec![]
    }
}
