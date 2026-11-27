use async_trait::async_trait;

use crate::ports::VoiceId;

/// A short recording of a participant's voice (any common audio container).
#[derive(Debug, Clone)]
pub struct VoiceSample {
    pub bytes: Vec<u8>,
    /// MIME type, e.g. `audio/webm`.
    pub content_type: String,
}

#[derive(Debug, thiserror::Error)]
pub enum VoiceCloneError {
    #[error("the provider rejected the voice sample: {0}")]
    Rejected(String),
    #[error("voice cloning request failed: {0}")]
    Request(String),
}

/// Instant voice cloning with explicit deletion (ADR 0006).
#[async_trait]
pub trait VoiceCloner: Send + Sync {
    async fn create(&self, name: &str, sample: VoiceSample) -> Result<VoiceId, VoiceCloneError>;
    async fn delete(&self, voice: &VoiceId) -> Result<(), VoiceCloneError>;
    /// Deletes every clone whose name starts with `prefix`; returns how many.
    /// Used at startup to remove clones orphaned by a crash or restart.
    async fn delete_all_named(&self, prefix: &str) -> Result<usize, VoiceCloneError>;
    /// Which provider holds the clones (`elevenlabs`, `inworld`), stored with kept clones.
    fn provider(&self) -> &'static str;
}
