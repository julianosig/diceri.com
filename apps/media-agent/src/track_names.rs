//! Naming contract shared with the web client (`apps/web/src/lib/features/call`).

use diceri_domain::LanguageCode;

/// Identity of the agent participant in every room.
pub(crate) const AGENT_IDENTITY: &str = "diceri-translator";
/// Data channel topic carrying caption JSON.
pub(crate) const CAPTIONS_TOPIC: &str = "diceri.captions";

/// Translated audio track: `tr:<speaker identity>:<language>`.
pub(crate) fn translated_track_name(speaker: &str, language: &LanguageCode) -> String {
    format!("tr:{speaker}:{language}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_speaker_and_language() {
        let name = translated_track_name("p-abc", &"pt-BR".parse().unwrap());
        assert_eq!(name, "tr:p-abc:pt-BR");
    }
}
