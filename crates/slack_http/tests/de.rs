use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
pub struct Test {
    #[serde(deserialize_with = "slack_http_types::offset_date_time_from_unix_ts")]
    pub test: OffsetDateTime,
}

#[test]
pub fn it_should_deserialize_unix_ts() {
    let ts = r#"{"test": 1449252889}"#;
    let _ = serde_json::from_str::<Test>(ts).unwrap();

    let ts = r#"{"test": 1716700028}"#;
    let _ = serde_json::from_str::<Test>(ts).unwrap();
}
