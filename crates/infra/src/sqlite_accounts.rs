use std::str::FromStr as _;
use std::time::Duration;

use async_trait::async_trait;
use diceri_application::ports::{
    AccountRepository, AccountRepositoryError, NewSession, PasswordHash, StoredAccount,
};
use diceri_domain::{AccessKey, Account, AccountId, Plan, Username};
use sha2::{Digest as _, Sha256};
use sqlx::migrate::Migrator;
use sqlx::sqlite::{
    SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions, SqliteSynchronous,
};

/// Versioned, forward-only schema (AGENTS.md §14), applied before serving.
static MIGRATIONS: Migrator = sqlx::migrate!("./migrations");

/// Accounts and sessions in SQLite. Passwords arrive already hashed (Argon2id);
/// session tokens are 128 random bits, so their SHA-256 is enough to make a leaked
/// database useless for signing in.
#[derive(Debug, Clone)]
pub struct SqliteAccountRepository {
    pool: SqlitePool,
}

fn storage(error: &dyn std::fmt::Display) -> AccountRepositoryError {
    AccountRepositoryError::Storage(error.to_string())
}

/// Every connection option in one place; change only with a written reason.
fn options(url: &str) -> Result<SqliteConnectOptions, AccountRepositoryError> {
    Ok(SqliteConnectOptions::from_str(url)
        .map_err(|e| storage(&e))?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        // FULL, not NORMAL: a sign-up lost on power failure would surprise its owner.
        .synchronous(SqliteSynchronous::Full)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5))
        .optimize_on_close(true, None))
}

impl SqliteAccountRepository {
    /// Opens (creating if needed) the database at `url`, e.g. `sqlite://diceri.db`
    /// or `sqlite::memory:`, and applies pending migrations.
    pub async fn connect(url: &str) -> Result<Self, AccountRepositoryError> {
        // An in-memory database lives in one connection; keep exactly that one open.
        let in_memory = url.contains(":memory:");
        let pool = SqlitePoolOptions::new()
            .max_connections(if in_memory { 1 } else { 4 })
            .idle_timeout(None)
            .max_lifetime(None)
            .connect_with(options(url)?)
            .await
            .map_err(|e| storage(&e))?;
        MIGRATIONS.run(&pool).await.map_err(|e| storage(&e))?;
        Ok(Self { pool })
    }

    /// Writes a consistent snapshot of the live database to `target` (which must not
    /// exist) with `VACUUM INTO`, then checks the copy with `PRAGMA integrity_check`.
    pub async fn snapshot(&self, target: &std::path::Path) -> Result<(), AccountRepositoryError> {
        let path = target
            .to_str()
            .ok_or_else(|| storage(&"backup path is not valid UTF-8"))?;
        sqlx::query("VACUUM INTO ?")
            .bind(path)
            .execute(&self.pool)
            .await
            .map_err(|e| storage(&e))?;
        let copy = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(SqliteConnectOptions::new().filename(target).read_only(true))
            .await
            .map_err(|e| storage(&e))?;
        let verdict: String = sqlx::query_scalar("PRAGMA integrity_check")
            .fetch_one(&copy)
            .await
            .map_err(|e| storage(&e))?;
        copy.close().await;
        if verdict == "ok" {
            Ok(())
        } else {
            Err(storage(&format!(
                "backup failed integrity_check: {verdict}"
            )))
        }
    }
}

fn token_hash(token: &AccessKey) -> Vec<u8> {
    Sha256::digest(token.expose().as_bytes()).to_vec()
}

fn millis(value: u64) -> i64 {
    i64::try_from(value).unwrap_or(i64::MAX)
}

/// Creation time is the instant encoded in the UUIDv7 id (Unix milliseconds).
fn created_at_ms(id: &AccountId) -> i64 {
    let (seconds, nanos) = id
        .as_uuid()
        .get_timestamp()
        .map_or((0, 0), |timestamp| timestamp.to_unix());
    let millis = u128::from(seconds) * 1000 + u128::from(nanos / 1_000_000);
    i64::try_from(millis).unwrap_or(i64::MAX)
}

/// Row shape of `accounts`, converted through the domain constructors.
#[derive(sqlx::FromRow)]
struct AccountRow {
    id: uuid::Uuid,
    username: String,
    mascot: String,
    plan: String,
    password_hash: String,
}

impl TryFrom<AccountRow> for StoredAccount {
    type Error = AccountRepositoryError;

    fn try_from(row: AccountRow) -> Result<Self, Self::Error> {
        Ok(Self {
            account: Account {
                id: AccountId::from_uuid(row.id).map_err(|e| storage(&e))?,
                username: row.username.parse().map_err(|e| storage(&e))?,
                mascot: row.mascot.parse().map_err(|e| storage(&e))?,
                plan: row.plan.parse().map_err(|e| storage(&e))?,
            },
            password_hash: PasswordHash(row.password_hash),
        })
    }
}

#[async_trait]
impl AccountRepository for SqliteAccountRepository {
    async fn create(&self, stored: &StoredAccount) -> Result<(), AccountRepositoryError> {
        let account = &stored.account;
        let result = sqlx::query(
            "INSERT INTO accounts (id, username, password_hash, mascot, plan, created_at) \
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(account.id.as_uuid())
        .bind(account.username.as_str())
        .bind(stored.password_hash.0.as_str())
        .bind(account.mascot.as_str())
        .bind(account.plan.as_str())
        .bind(created_at_ms(&account.id))
        .execute(&self.pool)
        .await;
        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(error)) if error.is_unique_violation() => {
                Err(AccountRepositoryError::Taken)
            }
            Err(error) => Err(storage(&error)),
        }
    }

    async fn find_by_username(
        &self,
        username: &Username,
    ) -> Result<Option<StoredAccount>, AccountRepositoryError> {
        // Served by `uq_accounts_username`.
        let row: Option<AccountRow> = sqlx::query_as(
            "SELECT id, username, mascot, plan, password_hash FROM accounts WHERE username = ?",
        )
        .bind(username.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| storage(&e))?;
        row.map(StoredAccount::try_from).transpose()
    }

    async fn create_session(&self, session: NewSession<'_>) -> Result<(), AccountRepositoryError> {
        // Expired sessions are purged here (served by `index_sessions_on_expires_at`).
        sqlx::query("DELETE FROM sessions WHERE expires_at <= ?")
            .bind(millis(session.created_at_ms))
            .execute(&self.pool)
            .await
            .map_err(|e| storage(&e))?;
        sqlx::query(
            "INSERT INTO sessions (id, account_id, token_hash, created_at, expires_at) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(session.id.as_uuid())
        .bind(session.account.as_uuid())
        .bind(token_hash(session.token))
        .bind(millis(session.created_at_ms))
        .bind(millis(session.expires_at_ms))
        .execute(&self.pool)
        .await
        .map_err(|e| storage(&e))?;
        Ok(())
    }

    async fn find_session(
        &self,
        token: &AccessKey,
        now_ms: u64,
    ) -> Result<Option<Account>, AccountRepositoryError> {
        // Served by `uq_sessions_token_hash` and the accounts primary key.
        let row: Option<AccountRow> = sqlx::query_as(
            "SELECT a.id, a.username, a.mascot, a.plan, a.password_hash \
             FROM sessions s JOIN accounts a ON a.id = s.account_id \
             WHERE s.token_hash = ? AND s.expires_at > ?",
        )
        .bind(token_hash(token))
        .bind(millis(now_ms))
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| storage(&e))?;
        Ok(row
            .map(StoredAccount::try_from)
            .transpose()?
            .map(|stored| stored.account))
    }

    async fn delete_session(&self, token: &AccessKey) -> Result<(), AccountRepositoryError> {
        sqlx::query("DELETE FROM sessions WHERE token_hash = ?")
            .bind(token_hash(token))
            .execute(&self.pool)
            .await
            .map_err(|e| storage(&e))?;
        Ok(())
    }

    async fn set_plan(
        &self,
        username: &Username,
        plan: Plan,
    ) -> Result<bool, AccountRepositoryError> {
        // Served by `uq_accounts_username`.
        let result = sqlx::query("UPDATE accounts SET plan = ? WHERE username = ?")
            .bind(plan.as_str())
            .bind(username.as_str())
            .execute(&self.pool)
            .await
            .map_err(|e| storage(&e))?;
        Ok(result.rows_affected() == 1)
    }
}

#[cfg(test)]
#[path = "sqlite_accounts_tests.rs"]
mod tests;
