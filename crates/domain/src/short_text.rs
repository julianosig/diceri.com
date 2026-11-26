//! Validation shared by short, user-typed labels (names, titles).

/// Why a label was rejected; each label type maps it to its own error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShortTextIssue {
    Empty,
    TooLong,
    ControlCharacter,
}

/// Trims `raw` and checks it is 1..=`max_chars` characters without control characters.
pub(crate) fn clean(raw: &str, max_chars: usize) -> Result<&str, ShortTextIssue> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(ShortTextIssue::Empty);
    }
    if trimmed.chars().count() > max_chars {
        return Err(ShortTextIssue::TooLong);
    }
    if trimmed.chars().any(char::is_control) {
        return Err(ShortTextIssue::ControlCharacter);
    }
    Ok(trimmed)
}
