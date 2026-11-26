use diceri_domain::{AccessKey, Account, AccountId, SessionId, Username};

use super::PasswordHash;

#[derive(Debug, thiserror::Error)]
pub enum AccountRepositoryError {
    #[error("username is taken")]
    Taken,
    #[error("account storage failed: {0}")]
    Storage(String),
}

/// An account as stored, with what is needed to check its password.
#[derive(Debug, Clone)]
pub struct StoredAccount {
    pub account: Account,
    pub password_hash: PasswordHash,
}

/// A signed-in browser. The token is a random bearer secret: implementations
/// store only its hash, never the token.
#[derive(Debug, Clone)]
pub struct NewSession<'a> {
    pub id: SessionId,
    pub account: &'a AccountId,
    pub token: &'a AccessKey,
    pub created_at_ms: u64,
    pub expires_at_ms: u64,
}

/// Durable accounts and their sessions.
#[async_trait::async_trait]
pub trait AccountRepository: Send + Sync {
    /// Fails with `Taken` when the username already exists.
    async fn create(&self, account: &StoredAccount) -> Result<(), AccountRepositoryError>;

    async fn find_by_username(
        &self,
        username: &Username,
    ) -> Result<Option<StoredAccount>, AccountRepositoryError>;

    async fn create_session(&self, session: NewSession<'_>) -> Result<(), AccountRepositoryError>;

    /// The account of a session that has not expired at `now_ms`.
    async fn find_session(
        &self,
        token: &AccessKey,
        now_ms: u64,
    ) -> Result<Option<Account>, AccountRepositoryError>;

    /// Ends one session; unknown tokens are fine.
    async fn delete_session(&self, token: &AccessKey) -> Result<(), AccountRepositoryError>;
}
