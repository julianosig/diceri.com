use diceri_domain::AccountId;

use super::{VoiceId, VoiceSample};

#[derive(Debug, thiserror::Error)]
pub enum VoiceSampleStoreError {
    #[error("voice sample storage failed: {0}")]
    Storage(String),
}

/// The clone of an account's voice kept at a provider (ADR 0017).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeptClone {
    /// Which provider holds it (`elevenlabs`, `inworld`): another provider cannot use it.
    pub provider: String,
    pub voice: VoiceId,
}

/// Each account's own voice recording, kept so its voice can be recreated with any
/// provider (ADR 0015). One per account; saving again replaces it. Implementations keep
/// it encrypted and never log it.
#[async_trait::async_trait]
pub trait VoiceSampleStore: Send + Sync {
    async fn save(
        &self,
        account: &AccountId,
        sample: &VoiceSample,
        recorded_at_ms: u64,
    ) -> Result<(), VoiceSampleStoreError>;

    async fn find(&self, account: &AccountId)
    -> Result<Option<VoiceSample>, VoiceSampleStoreError>;

    /// Whether the account has a recording, without reading it.
    async fn has(&self, account: &AccountId) -> Result<bool, VoiceSampleStoreError>;

    /// Deletes the account's recording and its kept clone; `false` when there was none.
    async fn delete(&self, account: &AccountId) -> Result<bool, VoiceSampleStoreError>;

    /// The account's voice clone kept at a provider, if any.
    async fn kept_clone(
        &self,
        account: &AccountId,
    ) -> Result<Option<KeptClone>, VoiceSampleStoreError>;

    /// Keeps `clone` as the account's voice clone, replacing an older one.
    async fn keep_clone(
        &self,
        account: &AccountId,
        clone: &KeptClone,
        created_at_ms: u64,
    ) -> Result<(), VoiceSampleStoreError>;
}
