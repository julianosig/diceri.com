use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use diceri_application::ports::{UsageRepository, UsageRepositoryError};
use diceri_domain::AccountId;

/// In-memory call minutes for tests and quick local runs.
#[derive(Debug, Default)]
pub struct MemoryUsageRepository {
    minutes: Mutex<HashMap<(AccountId, u32), u32>>,
}

fn poisoned<T>(_: T) -> UsageRepositoryError {
    UsageRepositoryError::Storage("usage store lock poisoned".to_owned())
}

#[async_trait]
impl UsageRepository for MemoryUsageRepository {
    async fn add_minute(&self, account: &AccountId, day: u32) -> Result<u32, UsageRepositoryError> {
        let mut minutes = self.minutes.lock().map_err(poisoned)?;
        let total = minutes.entry((*account, day)).or_insert(0);
        *total += 1;
        Ok(*total)
    }

    async fn minutes_on(&self, account: &AccountId, day: u32) -> Result<u32, UsageRepositoryError> {
        let minutes = self.minutes.lock().map_err(poisoned)?;
        Ok(minutes.get(&(*account, day)).copied().unwrap_or(0))
    }

    async fn total_between(
        &self,
        first_day: u32,
        last_day: u32,
    ) -> Result<u64, UsageRepositoryError> {
        let minutes = self.minutes.lock().map_err(poisoned)?;
        Ok(minutes
            .iter()
            .filter(|((_, day), _)| (first_day..=last_day).contains(day))
            .map(|(_, total)| u64::from(*total))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn honours_the_usage_repository_contract() {
        let account = AccountId::from_parts(1, [1; 10]);
        let other = AccountId::from_parts(2, [2; 10]);
        super::super::contract::check(&MemoryUsageRepository::default(), &account, &other).await;
    }
}
