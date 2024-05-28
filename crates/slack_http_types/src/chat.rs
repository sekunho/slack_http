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

    pub fn set_icon_emoji(self, icon_emoji: String) -> Self {
        let icon_emoji = format!(":{icon_emoji}:");

        Self {
            icon_emoji: Some(icon_emoji),
            ..self
        }
    }

    pub fn set_username(self, username: String) -> Self {
        Self {
            username: Some(username),
            ..self
        }
    }

    pub fn query_params(&self) -> Vec<(&str, &str)> {
        let mut opts = Vec::new();

        if let Some(username) = &self.username {
            opts.push(("username", username.as_str()))
        }

        if let Some(icon_emoji) = &self.icon_emoji {
            opts.push(("icon_emoji", icon_emoji.as_str()))
        }

        if let Some(icon_url) = &self.icon_url {
            opts.push(("icon_url", icon_url.as_str()))
        }

        opts.push(("link_names", if self.link_names { "true" } else { "false" }));
        opts.push(("mrkdwn", if self.markdown { "true" } else { "false" }));

        if let Some(unfurl_links) = self.unfurl_links {
            opts.push(("unfurl_links", if unfurl_links { "true" } else { "false" }));
        }

        if let Some(unfurl_media) = self.unfurl_media {
            opts.push(("unfurl_media", if unfurl_media { "true" } else { "false" }));
        }

        opts
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
