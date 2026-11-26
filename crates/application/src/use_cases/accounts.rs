use std::sync::Arc;

use diceri_domain::{AccessKey, Username};

use crate::ports::{AccountRepository, AccountRepositoryError, RandomSource};

#[derive(Debug, thiserror::Error)]
pub enum AccountError {
    #[error("this username is taken")]
    Taken,
    #[error("unknown or missing account key")]
    Unauthorized,
    #[error(transparent)]
    Repository(AccountRepositoryError),
}

impl From<AccountRepositoryError> for AccountError {
    fn from(error: AccountRepositoryError) -> Self {
        match error {
            AccountRepositoryError::Taken => Self::Taken,
            storage @ AccountRepositoryError::Storage(_) => Self::Repository(storage),
        }
    }
}

/// Username-only accounts: claiming a free username returns a secret key, which
/// is both the session and the code to sign in on another device.
#[derive(Clone)]
pub struct AccountService {
    accounts: Arc<dyn AccountRepository>,
    random: Arc<dyn RandomSource>,
}

impl std::fmt::Debug for AccountService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AccountService")
            .finish_non_exhaustive()
    }
}

impl AccountService {
    #[must_use]
    pub fn new(accounts: Arc<dyn AccountRepository>, random: Arc<dyn RandomSource>) -> Self {
        Self { accounts, random }
    }

    /// Claims `username`; the returned key must be handed to its owner once.
    pub async fn register(&self, username: &Username) -> Result<AccessKey, AccountError> {
        let key = AccessKey::from_random_bytes(self.random.random_128());
        self.accounts.create(username, &key).await?;
        Ok(key)
    }

    /// The account a key belongs to.
    pub async fn authenticate(&self, key: &AccessKey) -> Result<Username, AccountError> {
        self.accounts
            .find_by_key(key)
            .await?
            .ok_or(AccountError::Unauthorized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{CountingRandom, InMemoryAccounts};

    fn service() -> AccountService {
        AccountService::new(
            Arc::new(InMemoryAccounts::default()),
            Arc::new(CountingRandom::default()),
        )
    }

    fn name(raw: &str) -> Username {
        raw.parse().unwrap()
    }

    #[tokio::test]
    async fn the_key_signs_the_owner_in() {
        let accounts = service();
        let key = accounts.register(&name("ana")).await.unwrap();
        assert_eq!(accounts.authenticate(&key).await.unwrap(), name("ana"));
    }

    #[tokio::test]
    async fn usernames_are_unique() {
        let accounts = service();
        accounts.register(&name("ana")).await.unwrap();
        assert!(matches!(
            accounts.register(&name("ANA")).await,
            Err(AccountError::Taken)
        ));
    }

    #[tokio::test]
    async fn unknown_keys_are_unauthorized() {
        let unknown = AccessKey::from_random_bytes([9; 16]);
        assert!(matches!(
            service().authenticate(&unknown).await,
            Err(AccountError::Unauthorized)
        ));
    }
}
