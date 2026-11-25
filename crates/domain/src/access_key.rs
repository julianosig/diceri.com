use std::fmt;
use std::str::FromStr;

use crate::base62;

/// Secret bearer key (128 random bits) proving a role in a room: the host key,
/// or the key a guest receives when asking to join. Never log it.
#[derive(Clone, Eq)]
pub struct AccessKey(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("malformed access key")]
pub struct AccessKeyError;

impl AccessKey {
    #[must_use]
    pub fn from_random_bytes(bytes: [u8; 16]) -> Self {
        Self(base62::encode_128(bytes))
    }

    /// The raw key, for returning it once to its owner.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

/// Constant-time comparison so response timing does not leak key prefixes.
impl PartialEq for AccessKey {
    fn eq(&self, other: &Self) -> bool {
        let (left, right) = (self.0.as_bytes(), other.0.as_bytes());
        left.len() == right.len()
            && left.iter().zip(right).fold(0, |acc, (a, b)| acc | (a ^ b)) == 0
    }
}

impl fmt::Debug for AccessKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AccessKey(***)")
    }
}

impl FromStr for AccessKey {
    type Err = AccessKeyError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        base62::validate(raw).map_err(|_| AccessKeyError)?;
        Ok(Self(raw.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_output_is_redacted() {
        let key = AccessKey::from_random_bytes([9; 16]);
        assert_eq!(format!("{key:?}"), "AccessKey(***)");
    }

    #[test]
    fn compares_by_value() {
        let key = AccessKey::from_random_bytes([1; 16]);
        assert_eq!(key.expose().parse::<AccessKey>().unwrap(), key);
        assert_ne!(AccessKey::from_random_bytes([2; 16]), key);
    }
}
