use std::fmt;
use std::str::FromStr;

use crate::base62::{self, TokenFormatError};

/// Unguessable room identifier: 128 random bits encoded as 22 base62 characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoomId(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RoomIdError {
    #[error("room id must be exactly 22 characters")]
    InvalidLength,
    #[error("room id must contain only base62 characters")]
    InvalidCharacter,
}

impl RoomId {
    /// Builds a room id from 16 bytes of cryptographically secure randomness.
    #[must_use]
    pub fn from_random_bytes(bytes: [u8; 16]) -> Self {
        Self(base62::encode_128(bytes))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for RoomId {
    type Err = RoomIdError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        base62::validate(raw).map_err(|error| match error {
            TokenFormatError::Length => RoomIdError::InvalidLength,
            TokenFormatError::Character => RoomIdError::InvalidCharacter,
        })?;
        Ok(Self(raw.to_owned()))
    }
}

impl fmt::Display for RoomId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_string() {
        let id = RoomId::from_random_bytes([42; 16]);
        assert_eq!(id.as_str().parse::<RoomId>().unwrap(), id);
    }

    #[test]
    fn different_bytes_produce_different_ids() {
        assert_ne!(
            RoomId::from_random_bytes([1; 16]),
            RoomId::from_random_bytes([2; 16])
        );
    }

    #[test]
    fn rejects_malformed_ids() {
        assert_eq!("abc".parse::<RoomId>(), Err(RoomIdError::InvalidLength));
        assert_eq!(
            "abcdefghijklmnopqrstu-".parse::<RoomId>(),
            Err(RoomIdError::InvalidCharacter)
        );
    }
}
