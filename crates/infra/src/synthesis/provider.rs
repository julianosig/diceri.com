use std::str::FromStr;

/// Which provider clones voices and speaks translations (`DICERI_TTS_PROVIDER`). The API
/// (cloning at pre-join) and the media agent (speaking, deleting clones) must agree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TtsProvider {
    #[default]
    ElevenLabs,
    Inworld,
}

impl FromStr for TtsProvider {
    type Err = String;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        match raw {
            "elevenlabs" => Ok(Self::ElevenLabs),
            "inworld" => Ok(Self::Inworld),
            other => Err(format!("must be elevenlabs or inworld, got {other}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_supported_providers() {
        assert_eq!("inworld".parse(), Ok(TtsProvider::Inworld));
        assert_eq!("elevenlabs".parse(), Ok(TtsProvider::ElevenLabs));
        assert!("other".parse::<TtsProvider>().is_err());
    }
}
