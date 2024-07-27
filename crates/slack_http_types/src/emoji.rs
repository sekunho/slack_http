use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ListResponse {
    Ok { emoji: HashMap<String, String> },
    Error { error: String },
}
