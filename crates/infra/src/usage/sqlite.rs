use async_trait::async_trait;
use diceri_application::ports::{UsageRepository, UsageRepositoryError};
use diceri_domain::AccountId;
use sqlx::SqlitePool;

use crate::SqliteAccountRepository;

/// Call minutes in the accounts database (`call_minutes`), sharing its connection pool.
#[derive(Debug, Clone)]
pub struct SqliteUsageRepository {
    pool: SqlitePool,
}

impl SqliteUsageRepository {
    /// Uses the accounts database, whose migrations create `call_minutes`.
    #[must_use]
    pub fn sharing(accounts: &SqliteAccountRepository) -> Self {
        Self {
            pool: accounts.pool().clone(),
        }
    }
}

fn storage(error: &dyn std::fmt::Display) -> UsageRepositoryError {
    UsageRepositoryError::Storage(error.to_string())
}

fn whole(value: i64) -> u32 {
    u32::try_from(value).unwrap_or(0)
}

#[async_trait]
impl UsageRepository for SqliteUsageRepository {
    async fn add_minute(&self, account: &AccountId, day: u32) -> Result<u32, UsageRepositoryError> {
        // Served by `pk_call_minutes`.
        let (minutes,): (i64,) = sqlx::query_as(
            "INSERT INTO call_minutes (account_id, utc_day, minutes) VALUES (?, ?, 1) \
             ON CONFLICT (account_id, utc_day) DO UPDATE SET minutes = minutes + 1 \
             RETURNING minutes",
        )
        .bind(account.as_uuid())
        .bind(i64::from(day))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| storage(&e))?;
        Ok(whole(minutes))
    }

    async fn minutes_on(&self, account: &AccountId, day: u32) -> Result<u32, UsageRepositoryError> {
        // Served by `pk_call_minutes`.
        let row: Option<(i64,)> =
            sqlx::query_as("SELECT minutes FROM call_minutes WHERE account_id = ? AND utc_day = ?")
                .bind(account.as_uuid())
                .bind(i64::from(day))
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| storage(&e))?;
        Ok(row.map_or(0, |(minutes,)| whole(minutes)))
    }

    async fn total_between(
        &self,
        first_day: u32,
        last_day: u32,
    ) -> Result<u64, UsageRepositoryError> {
        // Served by `index_call_minutes_on_utc_day`.
        let (total,): (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(minutes), 0) FROM call_minutes WHERE utc_day BETWEEN ? AND ?",
        )
        .bind(i64::from(first_day))
        .bind(i64::from(last_day))
        .fetch_one(&self.pool)
        .await
        .map_err(|e| storage(&e))?;
        Ok(u64::try_from(total).unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use diceri_application::ports::AccountRepository as _;

    use super::*;
    use crate::account_contract::stored;

    #[tokio::test]
    async fn honours_the_usage_repository_contract() {
        let accounts = SqliteAccountRepository::connect("sqlite::memory:")
            .await
            .unwrap();
        let (ana, bob) = (stored(1, "ana"), stored(2, "bob"));
        accounts.create(&ana).await.unwrap();
        accounts.create(&bob).await.unwrap();
        let usage = SqliteUsageRepository::sharing(&accounts);
        super::super::contract::check(&usage, &ana.account.id, &bob.account.id).await;
    }
}
