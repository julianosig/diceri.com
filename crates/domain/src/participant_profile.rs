use crate::{DisplayName, LanguageCode};

/// How a participant presents themselves: their name and the language they
/// want to hear everyone in. The language they speak is detected, never declared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParticipantProfile {
    pub name: DisplayName,
    pub output_language: LanguageCode,
}
