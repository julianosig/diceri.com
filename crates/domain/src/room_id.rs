use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

const ALPHABET: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const ENCODED_LEN: usize = 22;

/// Unguessable room identifier: 128 random bits encoded as 22 base62 characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct RoomId(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RoomIdError {
    #[error("room id must be exactly {ENCODED_LEN} characters")]
    InvalidLength,
    #[error("room id must contain only base62 characters")]
    InvalidCharacter,
}

impl RoomId {
    /// Builds a room id from 16 bytes of cryptographically secure randomness.
    #[must_use]
    pub fn from_random_bytes(bytes: [u8; 16]) -> Self {
        let mut value = u128::from_be_bytes(bytes);
        let mut encoded = [b'0'; ENCODED_LEN];
        for slot in encoded.iter_mut().rev() {
            let digit = usize::try_from(value % 62).unwrap_or_default();
            *slot = ALPHABET.get(digit).copied().unwrap_or(b'0');
            value /= 62;
        }
        Self(encoded.iter().map(|&byte| char::from(byte)).collect())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for RoomId {
    type Err = RoomIdError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() != ENCODED_LEN {
            return Err(RoomIdError::InvalidLength);
        }
        if !raw.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
            return Err(RoomIdError::InvalidCharacter);
        }
        Ok(Self(raw.to_owned()))
    }
}

impl TryFrom<String> for RoomId {
    type Error = RoomIdError;

    fn try_from(raw: String) -> Result<Self, Self::Error> {
        raw.parse()
    }
}

impl From<RoomId> for String {
    fn from(id: RoomId) -> Self {
        id.0
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
    fn encodes_random_bytes_to_fixed_length() {
        let id = RoomId::from_random_bytes([255; 16]);
        assert_eq!(id.as_str().len(), ENCODED_LEN);
    }

    #[test]
    fn different_bytes_produce_different_ids() {
        let first = RoomId::from_random_bytes([1; 16]);
        let second = RoomId::from_random_bytes([2; 16]);
        assert_ne!(first, second);
    }

    #[test]
    fn round_trips_through_string() {
        let id = RoomId::from_random_bytes([42; 16]);
        let parsed: RoomId = id.as_str().parse().unwrap();
        assert_eq!(parsed, id);
    }

    #[test]
    fn rejects_wrong_length() {
        assert_eq!("abc".parse::<RoomId>(), Err(RoomIdError::InvalidLength));
    }

    #[test]
    fn rejects_non_base62_characters() {
        let raw = "abcdefghijklmnopqrstu-";
        assert_eq!(raw.parse::<RoomId>(), Err(RoomIdError::InvalidCharacter));
    }
}
