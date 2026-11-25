use std::fmt;
use std::str::FromStr;

const MAX_CHARS: usize = 40;

/// Name shown to other participants: trimmed, 1–40 characters, no control characters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayName(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum DisplayNameError {
    #[error("name must not be empty")]
    Empty,
    #[error("name must be at most {MAX_CHARS} characters")]
    TooLong,
    #[error("name must not contain control characters")]
    ControlCharacter,
}

impl DisplayName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for DisplayName {
    type Err = DisplayNameError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(DisplayNameError::Empty);
        }
        if trimmed.chars().count() > MAX_CHARS {
            return Err(DisplayNameError::TooLong);
        }
        if trimmed.chars().any(char::is_control) {
            return Err(DisplayNameError::ControlCharacter);
        }
        Ok(Self(trimmed.to_owned()))
    }
}

impl fmt::Display for DisplayName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_valid_names() {
        assert_eq!(
            "  Ana Souza ".parse::<DisplayName>().unwrap().as_str(),
            "Ana Souza"
        );
    }

    #[test]
    fn rejects_invalid_names() {
        assert_eq!("   ".parse::<DisplayName>(), Err(DisplayNameError::Empty));
        assert_eq!(
            "a".repeat(41).parse::<DisplayName>(),
            Err(DisplayNameError::TooLong)
        );
        assert_eq!(
            "a\u{7}b".parse::<DisplayName>(),
            Err(DisplayNameError::ControlCharacter)
        );
    }
}
