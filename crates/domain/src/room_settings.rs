use std::fmt;
use std::str::FromStr;

use crate::short_text::{ShortTextIssue, clean};

const MAX_TITLE_CHARS: usize = 60;

/// What a meeting is about, shown on the join screen and, if listed, the live board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomTitle(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum RoomTitleError {
    #[error("title must not be empty")]
    Empty,
    #[error("title must be at most {MAX_TITLE_CHARS} characters")]
    TooLong,
    #[error("title must not contain control characters")]
    ControlCharacter,
}

impl RoomTitle {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for RoomTitle {
    type Err = RoomTitleError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        clean(raw, MAX_TITLE_CHARS)
            .map(|text| Self(text.to_owned()))
            .map_err(|issue| match issue {
                ShortTextIssue::Empty => RoomTitleError::Empty,
                ShortTextIssue::TooLong => RoomTitleError::TooLong,
                ShortTextIssue::ControlCharacter => RoomTitleError::ControlCharacter,
            })
    }
}

impl fmt::Display for RoomTitle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// How people get in once they have the link (or find the room on the live board).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EntryPolicy {
    /// Anyone who asks is let in right away.
    Open,
    /// The host admits or denies each person.
    #[default]
    Approval,
}

/// Chosen by the host when creating the room.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomSettings {
    pub title: RoomTitle,
    /// Shown on the public live board (title and host name). Private by default.
    pub listed: bool,
    pub entry: EntryPolicy,
}

impl RoomSettings {
    /// Private, host-approved room: the defaults when the host chooses nothing.
    #[must_use]
    pub fn private(title: RoomTitle) -> Self {
        Self {
            title,
            listed: false,
            entry: EntryPolicy::Approval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn titles_are_trimmed_and_bounded() {
        assert_eq!(
            "  Spanish practice ".parse::<RoomTitle>().unwrap().as_str(),
            "Spanish practice"
        );
        assert_eq!(" ".parse::<RoomTitle>(), Err(RoomTitleError::Empty));
        assert_eq!(
            "a".repeat(61).parse::<RoomTitle>(),
            Err(RoomTitleError::TooLong)
        );
    }

    #[test]
    fn rooms_are_private_and_approved_by_default() {
        let settings = RoomSettings::private("Chat".parse().unwrap());
        assert!(!settings.listed);
        assert_eq!(settings.entry, EntryPolicy::Approval);
    }
}
