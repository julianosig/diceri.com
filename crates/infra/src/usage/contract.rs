//! One behaviour for every `UsageRepository` (AGENTS.md §14 rule 24).
#![allow(clippy::unwrap_used, reason = "tests fail loudly")]

use diceri_application::ports::UsageRepository;
use diceri_domain::AccountId;

/// `account` and `other` must be two existing, distinct accounts.
pub(crate) async fn check(usage: &dyn UsageRepository, account: &AccountId, other: &AccountId) {
    check_empty(usage, account).await;
    check_minutes(usage, account, other).await;
    check_totals(usage).await;
}

async fn check_empty(usage: &dyn UsageRepository, account: &AccountId) {
    // Nothing used yet.
    assert_eq!(
        usage
            .account_minutes_between(account, 0, 1_000)
            .await
            .unwrap(),
        0
    );
    assert_eq!(usage.total_between(0, 1_000).await.unwrap(), 0);
}

async fn check_minutes(usage: &dyn UsageRepository, account: &AccountId, other: &AccountId) {
    // Minutes add up per account and day, and each call returns the day's total.
    assert_eq!(usage.add_minute(account, 100).await.unwrap(), 1);
    assert_eq!(usage.add_minute(account, 100).await.unwrap(), 2);
    assert_eq!(usage.add_minute(account, 101).await.unwrap(), 1);
    assert_eq!(usage.add_minute(other, 100).await.unwrap(), 1);
    assert_eq!(
        usage
            .account_minutes_between(account, 100, 100)
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        usage
            .account_minutes_between(account, 100, 101)
            .await
            .unwrap(),
        3
    );
    assert_eq!(
        usage
            .account_minutes_between(other, 101, 200)
            .await
            .unwrap(),
        0
    );
}

/// Runs after `check_minutes`: 3 minutes on day 100, 1 on day 101.
async fn check_totals(usage: &dyn UsageRepository) {
    // Totals cover every account between two days, both included.
    assert_eq!(usage.total_between(100, 100).await.unwrap(), 3);
    assert_eq!(usage.total_between(100, 101).await.unwrap(), 4);
    assert_eq!(usage.total_between(102, 200).await.unwrap(), 0);
}
