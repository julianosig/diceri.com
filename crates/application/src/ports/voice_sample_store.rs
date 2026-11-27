use diceri_domain::AccountId;

use super::VoiceSample;

#[derive(Debug, thiserror::Error)]
pub enum VoiceSampleStoreError {
    #[error("voice sample storage failed: {0}")]
    Storage(String),
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

    /// Deletes the account's recording; `false` when there was none.
    async fn delete(&self, account: &AccountId) -> Result<bool, VoiceSampleStoreError>;
}
