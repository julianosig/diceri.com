//! Contract every `AccountRepository` adapter must honour, run against both the
//! in-memory and the SQLite implementation so they cannot drift apart.

use diceri_application::ports::{AccountRepository, AccountRepositoryError};
use diceri_domain::{AccessKey, AccountId, Username};

fn name(raw: &str) -> Username {
    raw.parse().unwrap()
}

fn key(seed: u8) -> AccessKey {
    AccessKey::from_random_bytes([seed; 16])
}

fn id(seed: u8) -> AccountId {
    AccountId::from_parts(u64::from(seed) + 1, [seed; 10])
}

pub(crate) async fn check(accounts: &dyn AccountRepository) {
    accounts
        .create(&id(1), &name("ana"), &key(1))
        .await
        .unwrap();

    // The key finds its account; unknown keys find nothing.
    assert_eq!(
        accounts.find_by_key(&key(1)).await.unwrap(),
        Some(name("ana"))
    );
    assert_eq!(accounts.find_by_key(&key(2)).await.unwrap(), None);

    // Usernames are unique, whatever the id and key.
    assert!(matches!(
        accounts.create(&id(2), &name("ana"), &key(2)).await,
        Err(AccountRepositoryError::Taken)
    ));
    assert_eq!(accounts.find_by_key(&key(2)).await.unwrap(), None);

    // Different usernames coexist.
    accounts
        .create(&id(3), &name("bob"), &key(3))
        .await
        .unwrap();
    assert_eq!(
        accounts.find_by_key(&key(3)).await.unwrap(),
        Some(name("bob"))
    );
}
