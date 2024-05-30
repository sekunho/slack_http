use std::num::ParseIntError;

use serde::{de, Deserialize, Deserializer};
use time::OffsetDateTime;

pub mod chat;
pub mod client;
pub mod conversation;
pub mod error;
pub mod oidc;
pub mod option;
pub mod page;
pub mod team;
pub mod user;
pub mod oauth;

/// Deserializes a UNIX timestamp with milliseconds into an `OffsetDateTime`.
pub fn offset_date_time_from_unix_ts<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: i64 = Deserialize::deserialize(deserializer)?;
    OffsetDateTime::from_unix_timestamp(ts).map_err(de::Error::custom)
}

/// Deserializes a UNIX timestamp with milliseconds into an `OffsetDateTime`.
pub fn offset_date_time_from_unix_ts_with_nano<'de, D>(
    deserializer: D,
) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let ts: String = Deserialize::deserialize(deserializer)?;

    let ts_chunks: Vec<i128> = ts
        .split('.')
        .map(|s| s.parse::<i128>())
        .collect::<Result<Vec<i128>, ParseIntError>>()
        .map_err(de::Error::custom)?;

    let ts_main = ts_chunks
        .first()
        .ok_or(de::Error::custom("invalid timestamp format"))?;

    let ts_milli = ts_chunks.get(1).ok_or(de::Error::custom(
        "expected nanoseconds in unix timestamp format",
    ))?;

    let ts = ts_main * 1_000_000_000 + ts_milli * 1_000;

    OffsetDateTime::from_unix_timestamp_nanos(ts).map_err(de::Error::custom)
}
