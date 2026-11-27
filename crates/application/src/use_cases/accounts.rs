use std::sync::Arc;

use diceri_domain::{AccessKey, Account, Mascot, Password, Plan, Username};

use crate::ports::{
    AccountRepository, AccountRepositoryError, Clock, IdGenerator, NewSession, PasswordHasher,
    PasswordHasherError, RandomSource, StoredAccount,
};

/// How long a browser stays signed in.
pub const SESSION_TTL_MS: u64 = 30 * 24 * 60 * 60 * 1000;

#[derive(Debug, thiserror::Error)]
pub enum AccountError {
    #[error("this username is taken")]
    Taken,
    /// Wrong username or password, or an unknown/expired session (deliberately vague).
    #[error("wrong username or password")]
    Unauthorized,
    #[error(transparent)]
    Repository(AccountRepositoryError),
    #[error(transparent)]
    Hasher(#[from] PasswordHasherError),
    #[error(transparent)]
    WeakPassword(#[from] diceri_domain::PasswordError),
}

impl From<AccountRepositoryError> for AccountError {
    fn from(error: AccountRepositoryError) -> Self {
        match error {
            AccountRepositoryError::Taken => Self::Taken,
            storage @ AccountRepositoryError::Storage(_) => Self::Repository(storage),
        }
    }
}

/// A signed-in browser: the account and the bearer token for its session cookie.
#[derive(Debug, Clone)]
pub struct SignedIn {
    pub account: Account,
    pub token: AccessKey,
}

/// What a person chooses in the single onboarding step.
#[derive(Debug, Clone)]
pub struct Registration {
    pub username: Username,
    pub password: Password,
    pub mascot: Mascot,
}

/// Username + password accounts (no email). The username is also the name of the
/// person's room. Passwords are hashed with the `PasswordHasher` port; sessions are
/// random tokens stored hashed.
#[derive(Clone)]
pub struct AccountService {
    accounts: Arc<dyn AccountRepository>,
    hasher: Arc<dyn PasswordHasher>,
    random: Arc<dyn RandomSource>,
    ids: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
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
    pub fn new(
        accounts: Arc<dyn AccountRepository>,
        hasher: Arc<dyn PasswordHasher>,
        random: Arc<dyn RandomSource>,
        ids: Arc<dyn IdGenerator>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            accounts,
            hasher,
            random,
            ids,
            clock,
        }
    }

    /// Creates the account and signs this browser in.
    pub async fn register(&self, registration: Registration) -> Result<SignedIn, AccountError> {
        registration
            .password
            .ensure_hard_to_guess(registration.username.as_str())?;
        let account = Account {
            id: self.ids.account_id(),
            username: registration.username,
            mascot: registration.mascot,
            // Everyone starts on the free plan.
            plan: Plan::Free,
        };
        let password_hash = self.hasher.hash(&registration.password).await?;
        self.accounts
            .create(&StoredAccount {
                account: account.clone(),
                password_hash,
            })
            .await?;
        self.start_session(account).await
    }

    /// Whether `username` is still free (usernames are public room addresses, so this
    /// reveals nothing a visit to `/@username` would not).
    pub async fn is_available(&self, username: &Username) -> Result<bool, AccountError> {
        Ok(self.accounts.find_by_username(username).await?.is_none())
    }

    /// The public face of `username` (handle and mascot), or `None` while unclaimed.
    /// Like `is_available`, it shows only what a visit to `/@username` shows.
    pub async fn profile(&self, username: &Username) -> Result<Option<Account>, AccountError> {
        Ok(self
            .accounts
            .find_by_username(username)
            .await?
            .map(|stored| stored.account))
    }

    pub async fn sign_in(
        &self,
        username: &Username,
        password: &Password,
    ) -> Result<SignedIn, AccountError> {
        let stored = self
            .accounts
            .find_by_username(username)
            .await?
            .ok_or(AccountError::Unauthorized)?;
        if !self.hasher.verify(password, &stored.password_hash).await? {
            return Err(AccountError::Unauthorized);
        }
        self.start_session(stored.account).await
    }

    /// The account behind a session cookie.
    pub async fn authenticate(&self, token: &AccessKey) -> Result<Account, AccountError> {
        self.accounts
            .find_session(token, self.clock.now_ms())
            .await?
            .ok_or(AccountError::Unauthorized)
    }

    pub async fn sign_out(&self, token: &AccessKey) -> Result<(), AccountError> {
        Ok(self.accounts.delete_session(token).await?)
    }

    async fn start_session(&self, account: Account) -> Result<SignedIn, AccountError> {
        let token = AccessKey::from_random_bytes(self.random.random_128());
        let now = self.clock.now_ms();
        self.accounts
            .create_session(NewSession {
                id: self.ids.session_id(),
                account: &account.id,
                token: &token,
                created_at_ms: now,
                expires_at_ms: now.saturating_add(SESSION_TTL_MS),
            })
            .await?;
        Ok(SignedIn { account, token })
    }
}

#[cfg(test)]
#[path = "accounts_tests.rs"]
mod tests;
