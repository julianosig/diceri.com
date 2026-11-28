//! How voice operations run: one at a time per account, with retried cleanup of unused
//! clones.

use std::collections::HashMap;
use std::sync::{Arc, PoisonError};

use diceri_domain::AccountId;

use crate::ports::{VoiceCloneError, VoiceCloner, VoiceId};

/// Attempts at deleting a clone that could not join its call, and the pause between them.
const DISCARD_ATTEMPTS: u32 = 3;
const DISCARD_RETRY_DELAY: std::time::Duration = std::time::Duration::from_secs(1);

#[derive(Default)]
pub(super) struct AccountLocks(std::sync::Mutex<HashMap<AccountId, Arc<tokio::sync::Mutex<()>>>>);

impl AccountLocks {
    pub(super) async fn lock(&self, account: &AccountId) -> tokio::sync::OwnedMutexGuard<()> {
        let lock = self
            .0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .entry(*account)
            .or_default()
            .clone();
        lock.lock_owned().await
    }
}

pub(super) async fn discard_with_retries(cloner: &dyn VoiceCloner, voice: &VoiceId) {
    for attempt in 1..=DISCARD_ATTEMPTS {
        match cloner.delete(voice).await {
            Ok(()) => return,
            Err(error) if attempt < DISCARD_ATTEMPTS => {
                warn_discard_retry(&error, attempt);
                tokio::time::sleep(DISCARD_RETRY_DELAY * attempt).await;
            }
            Err(error) => warn_discard_failed(&error, voice),
        }
    }
}

fn warn_discard_retry(error: &VoiceCloneError, attempt: u32) {
    tracing::warn!(%error, attempt, "unused voice clone not deleted, retrying");
}

fn warn_discard_failed(error: &VoiceCloneError, voice: &VoiceId) {
    tracing::error!(%error, voice_id = %voice.0, "unused voice clone left at the provider");
}
