use diceri_application::ports::VoiceId;

/// Picks a stable voice per speaker so each person always sounds the same.
pub(crate) fn voice_for(speaker: &str, voices: &[VoiceId]) -> VoiceId {
    let hash = speaker.bytes().fold(0_usize, |acc, byte| {
        acc.wrapping_mul(31).wrapping_add(usize::from(byte))
    });
    let index = hash.checked_rem(voices.len()).unwrap_or_default();
    voices
        .get(index)
        .cloned()
        .unwrap_or_else(|| VoiceId(String::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn voices() -> Vec<VoiceId> {
        ["a", "b", "c"].map(|id| VoiceId(id.to_owned())).to_vec()
    }

    #[test]
    fn same_speaker_always_gets_the_same_voice() {
        assert_eq!(voice_for("p-123", &voices()), voice_for("p-123", &voices()));
    }

    #[test]
    fn empty_voice_list_does_not_panic() {
        assert_eq!(voice_for("p-123", &[]), VoiceId(String::new()));
    }
}
