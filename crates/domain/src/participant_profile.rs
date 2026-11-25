use crate::{DisplayName, LanguageCode};

/// How a participant presents themselves: their name and the language they
/// speak (and want to hear others in).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParticipantProfile {
    pub name: DisplayName,
    pub language: LanguageCode,
}
