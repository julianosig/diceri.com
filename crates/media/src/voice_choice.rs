use std::sync::atomic::{AtomicBool, Ordering};

use diceri_application::ports::VoiceId;

/// The voice a speaker is translated in: only their own trained clone (ADR 0014). Without
/// one, or once the provider refuses it, their translations are captions only.
#[derive(Debug)]
pub(crate) struct VoiceChoice {
    clone: Option<VoiceId>,
    refused: AtomicBool,
}

impl VoiceChoice {
    pub(crate) fn new(clone: Option<VoiceId>) -> Self {
        Self {
            clone,
            refused: AtomicBool::new(false),
        }
    }

    /// The voice to speak in, or `None`: then nothing is spoken.
    pub(crate) fn current(&self) -> Option<&VoiceId> {
        self.clone
            .as_ref()
            .filter(|_| !self.refused.load(Ordering::Relaxed))
    }

    /// Stops speaking in the clone for good; true the first time.
    pub(crate) fn refuse(&self) -> bool {
        !self.refused.swap(true, Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speaks_only_in_the_clone_until_it_is_refused() {
        let choice = VoiceChoice::new(Some(VoiceId("clone".to_owned())));
        assert_eq!(choice.current(), Some(&VoiceId("clone".to_owned())));
        assert!(choice.refuse());
        assert_eq!(choice.current(), None);
        assert!(!choice.refuse());
    }

    #[test]
    fn without_a_trained_voice_nothing_is_spoken() {
        assert_eq!(VoiceChoice::new(None).current(), None);
    }
}
