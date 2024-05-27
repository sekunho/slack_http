use std::ops::Add;

use ring::hmac;
use thiserror::Error;
use time::OffsetDateTime;

pub mod chat;
pub mod client;
pub mod conversation;
pub mod emoji;
pub mod oauth;
pub mod oidc;
pub mod team;
pub mod user;

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("computed digest and slack's signature do not match")]
    DigestMismatch,
    #[error("provided slack signature isn't a valid hex")]
    SlackSignatureNotHex,
    #[error("timestamp is too old. should be <5mins")]
    TimestampTooOld,
    #[error("timestamp is not in a valid UNIX timestamp format")]
    InvalidTimestamp,
}

pub struct Cursor(pub(crate) String);

impl Cursor {
    pub fn get(&self) -> &str {
        self.0.as_str()
    }
}

/// Verifies if the request's body is from Slack.
pub fn verify(
    signing_secret: &[u8],
    timestamp: &str,
    signature: &str,
    message: &str,
) -> Result<(), VerificationError> {
    let key = hmac::Key::new(hmac::HMAC_SHA256, signing_secret);
    let basestring = format!("v0:{}:{}", timestamp, message);
    let digest = hmac::sign(&key, basestring.as_ref());

    let timestamp_num: i64 = timestamp
        .parse()
        .map_err(|_| VerificationError::InvalidTimestamp)?;

    // Check if the Slack timestamp is <5mins old.
    let slack_datetime = OffsetDateTime::from_unix_timestamp(timestamp_num)
        .map_err(|_| VerificationError::InvalidTimestamp)?;

    let now = OffsetDateTime::now_utc();
    let slack_datetime = slack_datetime.add(time::Duration::new(60 * 5, 0));

    if slack_datetime < now {
        return Err(VerificationError::TimestampTooOld);
    };

    let signature =
        ring::test::from_hex(signature).map_err(|_| VerificationError::SlackSignatureNotHex)?;

    if signature == digest.as_ref() {
        Ok(())
    } else {
        Err(VerificationError::DigestMismatch)
    }
}
