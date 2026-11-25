use std::fmt;
use std::str::FromStr;

use crate::base62;
use crate::{AccessKey, DisplayName};

/// Public identifier of a guest's request to join (safe to show the host).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestId(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("malformed request id")]
pub struct RequestIdError;

impl RequestId {
    #[must_use]
    pub fn from_random_bytes(bytes: [u8; 16]) -> Self {
        Self(base62::encode_128(bytes))
    }
}

impl FromStr for RequestId {
    type Err = RequestIdError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        base62::validate(raw).map_err(|_| RequestIdError)?;
        Ok(Self(raw.to_owned()))
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestStatus {
    Pending,
    Admitted,
    Denied,
}

/// A guest waiting in the lobby until the host admits or denies them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinRequest {
    pub id: RequestId,
    pub name: DisplayName,
    /// Proves the guest owns this request; returned only to the guest.
    pub key: AccessKey,
    pub status: RequestStatus,
}

impl JoinRequest {
    #[must_use]
    pub fn new(id: RequestId, name: DisplayName, key: AccessKey) -> Self {
        Self {
            id,
            name,
            key,
            status: RequestStatus::Pending,
        }
    }
}
