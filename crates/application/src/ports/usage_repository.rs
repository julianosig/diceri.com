use diceri_domain::AccountId;

#[derive(Debug, thiserror::Error)]
pub enum UsageRepositoryError {
    #[error("usage storage failed: {0}")]
    Storage(String),
}

/// Call minutes per account and UTC day (days since the Unix epoch). Minutes are charged
/// to the room's owner, and only while someone is in the call.
#[async_trait::async_trait]
pub trait UsageRepository: Send + Sync {
    /// Adds one minute to `account` on `day`; returns that day's new total.
    async fn add_minute(&self, account: &AccountId, day: u32) -> Result<u32, UsageRepositoryError>;

    /// The account's minutes from `first_day` to `last_day`, both included (0 without calls).
    async fn account_minutes_between(
        &self,
        account: &AccountId,
        first_day: u32,
        last_day: u32,
    ) -> Result<u32, UsageRepositoryError>;

    /// Every account's minutes from `first_day` to `last_day`, both included.
    async fn total_between(
        &self,
        first_day: u32,
        last_day: u32,
    ) -> Result<u64, UsageRepositoryError>;
}
