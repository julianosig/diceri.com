use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use diceri_application::ports::{KeptClone, VoiceSample, VoiceSampleStore, VoiceSampleStoreError};
use diceri_domain::AccountId;

/// Recordings in memory (tests, local runs without a voice key).
#[derive(Debug, Default)]
pub struct MemoryVoiceSamples {
    samples: Mutex<HashMap<AccountId, VoiceSample>>,
    clones: Mutex<HashMap<AccountId, KeptClone>>,
}

fn poisoned<T>(_: T) -> VoiceSampleStoreError {
    VoiceSampleStoreError::Storage("voice sample lock poisoned".to_owned())
}

#[async_trait]
impl VoiceSampleStore for MemoryVoiceSamples {
    async fn save(
        &self,
        account: &AccountId,
        sample: &VoiceSample,
        _recorded_at_ms: u64,
    ) -> Result<(), VoiceSampleStoreError> {
        self.samples
            .lock()
            .map_err(poisoned)?
            .insert(*account, sample.clone());
        Ok(())
    }

    async fn find(
        &self,
        account: &AccountId,
    ) -> Result<Option<VoiceSample>, VoiceSampleStoreError> {
        Ok(self.samples.lock().map_err(poisoned)?.get(account).cloned())
    }

    async fn has(&self, account: &AccountId) -> Result<bool, VoiceSampleStoreError> {
        Ok(self.samples.lock().map_err(poisoned)?.contains_key(account))
    }

    async fn delete(&self, account: &AccountId) -> Result<bool, VoiceSampleStoreError> {
        self.clones.lock().map_err(poisoned)?.remove(account);
        Ok(self
            .samples
            .lock()
            .map_err(poisoned)?
            .remove(account)
            .is_some())
    }

    async fn kept_clone(
        &self,
        account: &AccountId,
    ) -> Result<Option<KeptClone>, VoiceSampleStoreError> {
        Ok(self.clones.lock().map_err(poisoned)?.get(account).cloned())
    }

    async fn keep_clone(
        &self,
        account: &AccountId,
        clone: &KeptClone,
        _created_at_ms: u64,
    ) -> Result<(), VoiceSampleStoreError> {
        self.clones
            .lock()
            .map_err(poisoned)?
            .insert(*account, clone.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn honours_the_voice_sample_contract() {
        let account = AccountId::from_parts(1, [1; 10]);
        super::super::contract::check(&MemoryVoiceSamples::default(), &account).await;
    }
}
