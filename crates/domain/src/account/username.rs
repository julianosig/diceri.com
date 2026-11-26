use std::fmt;
use std::str::FromStr;

const MIN_CHARS: usize = 3;
const MAX_CHARS: usize = 20;
/// Names that would let someone pass as the service itself.
const RESERVED: &[&str] = &[
    "admin",
    "administrator",
    "diceri",
    "help",
    "host",
    "moderator",
    "official",
    "root",
    "staff",
    "support",
    "system",
];

/// Public account handle, shown as `@name`: 3–20 of `a-z`, `0-9` and `_`, starting
/// with a letter. Input is case-insensitive and stored lowercase, so `Ana` and
/// `ana` are the same account.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Username(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum UsernameError {
    #[error("username must be {MIN_CHARS} to {MAX_CHARS} characters")]
    Length,
    #[error("username must start with a letter and use only letters, numbers and _")]
    Characters,
    #[error("this username is reserved")]
    Reserved,
}

impl Username {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Username {
    type Err = UsernameError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let name = raw.trim().trim_start_matches('@').to_ascii_lowercase();
        if !(MIN_CHARS..=MAX_CHARS).contains(&name.chars().count()) {
            return Err(UsernameError::Length);
        }
        let mut chars = name.chars();
        let starts_with_letter = chars.next().is_some_and(|c| c.is_ascii_lowercase());
        let rest_valid = chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
        if !(starts_with_letter && rest_valid) {
            return Err(UsernameError::Characters);
        }
        if RESERVED.contains(&name.as_str()) {
            return Err(UsernameError::Reserved);
        }
        Ok(Self(name))
    }
}

impl fmt::Display for Username {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_case_and_a_leading_at() {
        assert_eq!(" @Ana_92 ".parse::<Username>().unwrap().as_str(), "ana_92");
    }

    #[test]
    fn rejects_bad_lengths_characters_and_reserved_names() {
        assert_eq!("ab".parse::<Username>(), Err(UsernameError::Length));
        assert_eq!(
            "a".repeat(21).parse::<Username>(),
            Err(UsernameError::Length)
        );
        assert_eq!("9lives".parse::<Username>(), Err(UsernameError::Characters));
        assert_eq!(
            "ana silva".parse::<Username>(),
            Err(UsernameError::Characters)
        );
        assert_eq!("joão".parse::<Username>(), Err(UsernameError::Characters));
        assert_eq!("Admin".parse::<Username>(), Err(UsernameError::Reserved));
    }
}
