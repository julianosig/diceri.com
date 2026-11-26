use std::fmt;

use diceri_domain::Password;

/// A password hash in PHC string format (`$argon2id$v=19$...`); opaque to the app.
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordHash(pub String);

impl fmt::Debug for PasswordHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PasswordHash(***)")
    }
}

#[derive(Debug, thiserror::Error)]
#[error("password hashing failed: {0}")]
pub struct PasswordHasherError(pub String);

/// Slow, salted password hashing (Argon2id in production). Implementations must
/// not block the async runtime.
#[async_trait::async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: &Password) -> Result<PasswordHash, PasswordHasherError>;

    /// `false` for a wrong password; errors only for a malformed hash.
    async fn verify(
        &self,
        password: &Password,
        hash: &PasswordHash,
    ) -> Result<bool, PasswordHasherError>;
}

/// Current time, so expiry checks are testable.
pub trait Clock: Send + Sync {
    fn now_ms(&self) -> u64;
}
