use std::sync::Mutex;

use async_trait::async_trait;
use diceri_application::ports::{
    AccountRepository, AccountRepositoryError, NewSession, StoredAccount,
};
use diceri_domain::{AccessKey, Account, AccountId, NetworkId, Plan, Username};
use sha2::{Digest as _, Sha256};

/// In-memory accounts and sessions for tests and quick local runs; hashes session
/// tokens like the real store.
#[derive(Debug, Default)]
pub struct MemoryAccountRepository {
    accounts: Mutex<Vec<StoredAccount>>,
    /// Networks accounts were signed up from (one account each).
    networks: Mutex<Vec<NetworkId>>,
    /// `(token hash, account id, expires at)`.
    sessions: Mutex<Vec<(Vec<u8>, AccountId, u64)>>,
}

fn token_hash(token: &AccessKey) -> Vec<u8> {
    Sha256::digest(token.expose().as_bytes()).to_vec()
}

fn poisoned<T>(_: T) -> AccountRepositoryError {
    AccountRepositoryError::Storage("account store lock poisoned".to_owned())
}

#[async_trait]
impl AccountRepository for MemoryAccountRepository {
    async fn create_from(
        &self,
        stored: &StoredAccount,
        network: Option<&NetworkId>,
    ) -> Result<(), AccountRepositoryError> {
        let mut accounts = self.accounts.lock().map_err(poisoned)?;
        let mut networks = self.networks.lock().map_err(poisoned)?;
        if accounts
            .iter()
            .any(|other| other.account.username == stored.account.username)
        {
            return Err(AccountRepositoryError::Taken);
        }
        if let Some(network) = network {
            if networks.contains(network) {
                return Err(AccountRepositoryError::NetworkUsed);
            }
            networks.push(*network);
        }
        accounts.push(stored.clone());
        Ok(())
    }

    async fn find_by_username(
        &self,
        username: &Username,
    ) -> Result<Option<StoredAccount>, AccountRepositoryError> {
        let accounts = self.accounts.lock().map_err(poisoned)?;
        Ok(accounts
            .iter()
            .find(|stored| &stored.account.username == username)
            .cloned())
    }

    async fn create_session(&self, session: NewSession<'_>) -> Result<(), AccountRepositoryError> {
        let mut sessions = self.sessions.lock().map_err(poisoned)?;
        sessions.retain(|(_, _, expires)| *expires > session.created_at_ms);
        sessions.push((
            token_hash(session.token),
            *session.account,
            session.expires_at_ms,
        ));
        Ok(())
    }

    async fn find_session(
        &self,
        token: &AccessKey,
        now_ms: u64,
    ) -> Result<Option<Account>, AccountRepositoryError> {
        let hash = token_hash(token);
        let account_id = self
            .sessions
            .lock()
            .map_err(poisoned)?
            .iter()
            .find(|(stored, _, expires)| *stored == hash && *expires > now_ms)
            .map(|(_, id, _)| *id);
        let accounts = self.accounts.lock().map_err(poisoned)?;
        Ok(account_id.and_then(|id| {
            accounts
                .iter()
                .find(|stored| stored.account.id == id)
                .map(|stored| stored.account.clone())
        }))
    }

    async fn delete_session(&self, token: &AccessKey) -> Result<(), AccountRepositoryError> {
        let hash = token_hash(token);
        self.sessions
            .lock()
            .map_err(poisoned)?
            .retain(|(stored, _, _)| *stored != hash);
        Ok(())
    }

    async fn set_plan(
        &self,
        username: &Username,
        plan: Plan,
    ) -> Result<bool, AccountRepositoryError> {
        let mut accounts = self.accounts.lock().map_err(poisoned)?;
        let found = accounts
            .iter_mut()
            .find(|stored| &stored.account.username == username);
        Ok(found.map(|stored| stored.account.plan = plan).is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_contract;

    #[tokio::test]
    async fn honours_the_account_repository_contract() {
        account_contract::check(&MemoryAccountRepository::default()).await;
    }
}
