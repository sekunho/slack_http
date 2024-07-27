use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ListResponse {
    Ok { emoji: HashMap<String, String> },
    Error { error: String },
}
