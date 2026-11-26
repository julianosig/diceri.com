use std::str::FromStr as _;
use std::time::Duration;

use async_trait::async_trait;
use diceri_application::ports::{AccountRepository, AccountRepositoryError};
use diceri_domain::{AccessKey, Username};
use sha2::{Digest as _, Sha256};
use sqlx::Row as _;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS accounts (
    username   TEXT PRIMARY KEY NOT NULL,
    key_hash   BLOB NOT NULL UNIQUE,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT";

/// Accounts in SQLite. Keys are 128 random bits, so a plain SHA-256 is enough to
/// make a leaked database useless for signing in (no salt or slow hash needed).
#[derive(Debug, Clone)]
pub struct SqliteAccountRepository {
    pool: SqlitePool,
}

fn storage(error: &dyn std::fmt::Display) -> AccountRepositoryError {
    AccountRepositoryError::Storage(error.to_string())
}

impl SqliteAccountRepository {
    /// Opens (creating if needed) the database at `url`, e.g. `sqlite://diceri.db`
    /// or `sqlite::memory:`, and makes sure the schema exists.
    pub async fn connect(url: &str) -> Result<Self, AccountRepositoryError> {
        let options = SqliteConnectOptions::from_str(url)
            .map_err(|e| storage(&e))?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_secs(5));
        // An in-memory database lives in one connection; keep exactly that one open.
        let in_memory = url.contains(":memory:");
        let pool = SqlitePoolOptions::new()
            .max_connections(if in_memory { 1 } else { 4 })
            .idle_timeout(None)
            .max_lifetime(None)
            .connect_with(options)
            .await
            .map_err(|e| storage(&e))?;
        sqlx::query(SCHEMA)
            .execute(&pool)
            .await
            .map_err(|e| storage(&e))?;
        Ok(Self { pool })
    }
}

fn key_hash(key: &AccessKey) -> Vec<u8> {
    Sha256::digest(key.expose().as_bytes()).to_vec()
}

#[async_trait]
impl AccountRepository for SqliteAccountRepository {
    async fn create(
        &self,
        username: &Username,
        key: &AccessKey,
    ) -> Result<(), AccountRepositoryError> {
        let result = sqlx::query("INSERT INTO accounts (username, key_hash) VALUES (?, ?)")
            .bind(username.as_str())
            .bind(key_hash(key))
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

    async fn repository() -> SqliteAccountRepository {
        SqliteAccountRepository::connect("sqlite::memory:")
            .await
            .unwrap()
    }

    fn name(raw: &str) -> Username {
        raw.parse().unwrap()
    }

    #[tokio::test]
    async fn finds_the_account_by_its_key() {
        let accounts = repository().await;
        let key = AccessKey::from_random_bytes([1; 16]);
        accounts.create(&name("ana"), &key).await.unwrap();
        assert_eq!(accounts.find_by_key(&key).await.unwrap(), Some(name("ana")));
        let other = AccessKey::from_random_bytes([2; 16]);
        assert_eq!(accounts.find_by_key(&other).await.unwrap(), None);
    }

    #[tokio::test]
    async fn usernames_are_unique() {
        let accounts = repository().await;
        let first = AccessKey::from_random_bytes([1; 16]);
        accounts.create(&name("ana"), &first).await.unwrap();
        let second = AccessKey::from_random_bytes([2; 16]);
        assert!(matches!(
            accounts.create(&name("ana"), &second).await,
            Err(AccountRepositoryError::Taken)
        ));
    }

    #[tokio::test]
    async fn keys_are_stored_only_as_hashes() {
        let accounts = repository().await;
        let key = AccessKey::from_random_bytes([3; 16]);
        accounts.create(&name("bob"), &key).await.unwrap();
        let stored: Vec<u8> = sqlx::query("SELECT key_hash FROM accounts")
            .fetch_one(&accounts.pool)
            .await
            .unwrap()
            .get("key_hash");
        assert_ne!(stored, key.expose().as_bytes());
        assert_eq!(stored.len(), 32);
    }
}
