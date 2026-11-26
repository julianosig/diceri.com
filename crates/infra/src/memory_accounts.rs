use std::sync::Mutex;

use async_trait::async_trait;
use diceri_application::ports::{AccountRepository, AccountRepositoryError};
use diceri_domain::{AccessKey, Username};
use sha2::{Digest as _, Sha256};

/// In-memory accounts for tests and quick local runs; hashes keys like the real store.
#[derive(Debug, Default)]
pub struct MemoryAccountRepository {
    accounts: Mutex<Vec<(Username, Vec<u8>)>>,
}

fn key_hash(key: &AccessKey) -> Vec<u8> {
    Sha256::digest(key.expose().as_bytes()).to_vec()
}

fn poisoned<T>(_: T) -> AccountRepositoryError {
    AccountRepositoryError::Storage("account store lock poisoned".to_owned())
}

#[async_trait]
impl AccountRepository for MemoryAccountRepository {
    async fn create(
        &self,
        username: &Username,
        key: &AccessKey,
    ) -> Result<(), AccountRepositoryError> {
        let mut accounts = self.accounts.lock().map_err(poisoned)?;
        if accounts.iter().any(|(name, _)| name == username) {
            return Err(AccountRepositoryError::Taken);
        }
        accounts.push((username.clone(), key_hash(key)));
        Ok(())
    }

    async fn find_by_key(
        &self,
        key: &AccessKey,
    ) -> Result<Option<Username>, AccountRepositoryError> {
        let hash = key_hash(key);
        let accounts = self.accounts.lock().map_err(poisoned)?;
        Ok(accounts
            .iter()
            .find(|(_, stored)| *stored == hash)
            .map(|(name, _)| name.clone()))
    }
}
