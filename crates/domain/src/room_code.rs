use std::fmt;
use std::str::FromStr;

use crate::base62::{self, TokenFormatError};

/// The room's shareable link (`/r/<code>`): 128 random bits as 22 base62
/// characters. Knowing it is enough to reach the room, so it is never an id,
/// never logged and never sent to the media server; use `RoomId` for that.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RoomCode(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RoomCodeError {
    #[error("room code must be exactly 22 characters")]
    InvalidLength,
    #[error("room code must contain only base62 characters")]
    InvalidCharacter,
}

impl RoomCode {
    /// Builds a room code from 16 bytes of cryptographically secure randomness.
    #[must_use]
    pub fn from_random_bytes(bytes: [u8; 16]) -> Self {
        Self(base62::encode_128(bytes))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for RoomCode {
    type Err = RoomCodeError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        base62::validate(raw).map_err(|error| match error {
            TokenFormatError::Length => RoomCodeError::InvalidLength,
            TokenFormatError::Character => RoomCodeError::InvalidCharacter,
        })?;
        Ok(Self(raw.to_owned()))
    }
}

impl fmt::Display for RoomCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Redacted: a code in a log line would hand out access to the room.
impl fmt::Debug for RoomCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RoomCode(***)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_string() {
        let code = RoomCode::from_random_bytes([42; 16]);
        assert_eq!(code.as_str().parse::<RoomCode>().unwrap(), code);
    }

    #[test]
    fn different_bytes_produce_different_codes() {
        assert_ne!(
            RoomCode::from_random_bytes([1; 16]),
            RoomCode::from_random_bytes([2; 16])
        );
    }

    #[test]
    fn rejects_malformed_codes() {
        assert_eq!("abc".parse::<RoomCode>(), Err(RoomCodeError::InvalidLength));
        assert_eq!(
            "abcdefghijklmnopqrstu-".parse::<RoomCode>(),
            Err(RoomCodeError::InvalidCharacter)
        );
    }

    #[test]
    fn debug_output_hides_the_code() {
        let code = RoomCode::from_random_bytes([7; 16]);
        assert_eq!(format!("{code:?}"), "RoomCode(***)");
    }
}
