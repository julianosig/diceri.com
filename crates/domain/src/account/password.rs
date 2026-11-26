use std::fmt;
use std::str::FromStr;

const MIN_CHARS: usize = 8;
/// Bounds the cost of hashing attacker-supplied input.
const MAX_CHARS: usize = 128;

/// A user-chosen password, validated for length only (NIST SP 800-63B: no
/// composition rules). It is only ever hashed (Argon2id); never stored or logged.
#[derive(Clone, PartialEq, Eq)]
pub struct Password(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PasswordError {
    #[error("password must be at least {MIN_CHARS} characters")]
    TooShort,
    #[error("password must be at most {MAX_CHARS} characters")]
    TooLong,
}

impl Password {
    /// The raw password, for the hasher only.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl FromStr for Password {
    type Err = PasswordError;

    /// Kept as typed: spaces and any characters are allowed and significant.
    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let length = raw.chars().count();
        if length < MIN_CHARS {
            return Err(PasswordError::TooShort);
        }
        if length > MAX_CHARS {
            return Err(PasswordError::TooLong);
        }
        Ok(Self(raw.to_owned()))
    }
}

impl fmt::Debug for Password {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Password(***)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_length_only_and_keeps_the_text() {
        assert_eq!("short".parse::<Password>(), Err(PasswordError::TooShort));
        assert_eq!(
            "x".repeat(129).parse::<Password>(),
            Err(PasswordError::TooLong)
        );
        let password: Password = " correct horse ".parse().unwrap();
        assert_eq!(password.expose(), " correct horse ");
        assert_eq!(format!("{password:?}"), "Password(***)");
    }
}
