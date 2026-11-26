use std::str::FromStr as _;
use std::time::Duration;

use async_trait::async_trait;
use diceri_application::ports::{AccountRepository, AccountRepositoryError};
use diceri_domain::{AccessKey, AccountId, Username};
use sha2::{Digest as _, Sha256};
use sqlx::Row as _;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{
    SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions, SqliteSynchronous,
};

/// Versioned, forward-only schema (AGENTS.md §14), applied before serving.
static MIGRATIONS: Migrator = sqlx::migrate!("./migrations");

/// Accounts in SQLite. Keys are 128 random bits, so a plain SHA-256 is enough to
/// make a leaked database useless for signing in (no salt or slow hash needed).
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
        // FULL, not NORMAL: a sign-up lost on power failure would leave its owner
        // holding a key that matches no account.
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
}

impl SqliteAccountRepository {
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

fn key_hash(key: &AccessKey) -> Vec<u8> {
    Sha256::digest(key.expose().as_bytes()).to_vec()
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

#[async_trait]
impl AccountRepository for SqliteAccountRepository {
    async fn create(
        &self,
        id: &AccountId,
        username: &Username,
        key: &AccessKey,
    ) -> Result<(), AccountRepositoryError> {
        let result = sqlx::query(
            "INSERT INTO accounts (id, username, key_hash, created_at) VALUES (?, ?, ?, ?)",
        )
        .bind(id.as_uuid())
        .bind(username.as_str())
        .bind(key_hash(key))
        .bind(created_at_ms(id))
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

    async fn find_by_key(
        &self,
        key: &AccessKey,
    ) -> Result<Option<Username>, AccountRepositoryError> {
        // Covered by the unique index `uq_accounts_key_hash`.
        let row = sqlx::query("SELECT username FROM accounts WHERE key_hash = ?")
            .bind(key_hash(key))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| storage(&e))?;
        row.map(|row| {
            let name: String = row.try_get("username").map_err(|e| storage(&e))?;
            name.parse().map_err(|e| storage(&e))
        })
        .transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_contract;

    async fn repository() -> SqliteAccountRepository {
        SqliteAccountRepository::connect("sqlite::memory:")
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn honours_the_account_repository_contract() {
        account_contract::check(&repository().await).await;
    }

    #[tokio::test]
    async fn stores_v7_ids_hashed_keys_and_millisecond_times() {
        let accounts = repository().await;
        let id = AccountId::from_parts(1_764_000_000_123, [1; 10]);
        let key = AccessKey::from_random_bytes([3; 16]);
        accounts
            .create(&id, &"bob".parse().unwrap(), &key)
            .await
            .unwrap();
        let row = sqlx::query("SELECT id, key_hash, created_at FROM accounts")
            .fetch_one(&accounts.pool)
            .await
            .unwrap();
        let stored_id: uuid::Uuid = row.get("id");
        let stored_hash: Vec<u8> = row.get("key_hash");
        let created_at: i64 = row.get("created_at");
        assert_eq!(stored_id, id.as_uuid());
        assert_eq!(stored_hash.len(), 32);
        assert_ne!(stored_hash, key.expose().as_bytes());
        assert_eq!(created_at, 1_764_000_000_123);
    }

    #[tokio::test]
    async fn snapshots_are_complete_and_checked() {
        let dir = std::env::temp_dir().join(format!("diceri-backup-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let live = dir.join("live.db");
        let accounts = SqliteAccountRepository::connect(&format!("sqlite://{}", live.display()))
            .await
            .unwrap();
        let key = AccessKey::from_random_bytes([4; 16]);
        let id = AccountId::from_parts(5, [5; 10]);
        accounts
            .create(&id, &"carla".parse().unwrap(), &key)
            .await
            .unwrap();

        let copy = dir.join("copy.db");
        accounts.snapshot(&copy).await.unwrap();
        let restored = SqliteAccountRepository::connect(&format!("sqlite://{}", copy.display()))
            .await
            .unwrap();
        assert_eq!(
            restored.find_by_key(&key).await.unwrap(),
            Some("carla".parse().unwrap())
        );
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[tokio::test]
    async fn the_schema_rejects_rows_the_domain_would_reject() {
        let accounts = repository().await;
        let id = AccountId::from_parts(1, [1; 10]);
        let insert = sqlx::query(
            "INSERT INTO accounts (id, username, key_hash, created_at) VALUES (?, 'Ana', ?, 1)",
        )
        .bind(id.as_uuid())
        .bind(vec![0_u8; 32])
        .execute(&accounts.pool)
        .await;
        let error = insert.unwrap_err();
        assert!(
            error
                .as_database_error()
                .is_some_and(sqlx::error::DatabaseError::is_check_violation)
        );
    }
}
