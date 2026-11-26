use diceri_domain::{AccessKey, Username};

#[derive(Debug, thiserror::Error)]
pub enum AccountRepositoryError {
    #[error("username is taken")]
    Taken,
    #[error("account storage failed: {0}")]
    Storage(String),
}

/// Durable accounts. An account is a username plus a secret key that signs its
/// owner in; implementations must store only a hash of the key, never the key.
#[async_trait::async_trait]
pub trait AccountRepository: Send + Sync {
    /// Fails with `Taken` when the username already exists.
    async fn create(
        &self,
        username: &Username,
        key: &AccessKey,
    ) -> Result<(), AccountRepositoryError>;

    async fn find_by_key(
        &self,
        key: &AccessKey,
    ) -> Result<Option<Username>, AccountRepositoryError>;
}
