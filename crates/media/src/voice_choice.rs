use std::sync::atomic::{AtomicBool, Ordering};

use diceri_application::ports::VoiceId;

/// The voice a speaker is translated in: their clone when they recorded one,
/// otherwise (or once the clone is refused) a stock voice.
#[derive(Debug)]
pub(crate) struct VoiceChoice {
    preferred: VoiceId,
    fallback: VoiceId,
    preferred_failed: AtomicBool,
}

impl VoiceChoice {
    pub(crate) fn new(preferred: VoiceId, fallback: VoiceId) -> Self {
        Self {
            preferred,
            fallback,
            preferred_failed: AtomicBool::new(false),
        }
    }

    pub(crate) fn current(&self) -> &VoiceId {
        if self.preferred_failed.load(Ordering::Relaxed) {
            &self.fallback
        } else {
            &self.preferred
        }
    }

    /// Switches to the fallback for good; returns false if there is nothing to switch to.
    pub(crate) fn give_up_preferred(&self) -> bool {
        if self.preferred == self.fallback {
            return false;
        }
        !self.preferred_failed.swap(true, Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn voice(id: &str) -> VoiceId {
        VoiceId(id.to_owned())
    }

    #[test]
    fn falls_back_once_and_stays_there() {
        let choice = VoiceChoice::new(voice("clone"), voice("stock"));
        assert_eq!(choice.current(), &voice("clone"));
        assert!(choice.give_up_preferred());
        assert_eq!(choice.current(), &voice("stock"));
        assert!(!choice.give_up_preferred());
    }

    #[test]
    fn nothing_to_fall_back_to_when_already_stock() {
        let choice = VoiceChoice::new(voice("stock"), voice("stock"));
        assert!(!choice.give_up_preferred());
    }
}
