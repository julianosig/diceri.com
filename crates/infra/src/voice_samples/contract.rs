//! One behaviour for every `VoiceSampleStore` (AGENTS.md §14 rule 24).
#![allow(clippy::unwrap_used, reason = "tests fail loudly")]

use diceri_application::ports::{VoiceSample, VoiceSampleStore};
use diceri_domain::AccountId;

fn sample(byte: u8) -> VoiceSample {
    VoiceSample {
        bytes: vec![byte; 64],
        content_type: "audio/wav".to_owned(),
    }
}

/// `account` must be an existing account.
pub(crate) async fn check(store: &dyn VoiceSampleStore, account: &AccountId) {
    check_empty(store, account).await;
    check_saved(store, account).await;
}

async fn check_empty(store: &dyn VoiceSampleStore, account: &AccountId) {
    assert!(!store.has(account).await.unwrap());
    assert!(store.find(account).await.unwrap().is_none());
    assert!(!store.delete(account).await.unwrap());
}

async fn check_saved(store: &dyn VoiceSampleStore, account: &AccountId) {
    store.save(account, &sample(1), 10).await.unwrap();
    assert!(store.has(account).await.unwrap());
    // Saving again replaces the recording.
    store.save(account, &sample(2), 20).await.unwrap();
    let found = store.find(account).await.unwrap().unwrap();
    assert_eq!(found.bytes, vec![2; 64]);
    assert_eq!(found.content_type, "audio/wav");

    assert!(store.delete(account).await.unwrap());
    assert!(!store.has(account).await.unwrap());
}
