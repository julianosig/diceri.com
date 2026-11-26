//! `diceri-api backup <file>`: a checked snapshot of the accounts database.
//! Runs beside the live server (`VACUUM INTO` does not block writers); used by
//! `scripts/backup-db.sh`.

use std::path::Path;

use anyhow::Context as _;
use diceri_infra::SqliteAccountRepository;

use crate::Config;

/// Writes the snapshot to `target`, which must not exist yet.
pub async fn run(config: &Config, target: &Path) -> anyhow::Result<()> {
    anyhow::ensure!(!target.exists(), "{} already exists", target.display());
    let accounts = SqliteAccountRepository::connect(&config.database_url)
        .await
        .context("cannot open the accounts database")?;
    accounts.snapshot(target).await.context("snapshot failed")?;
    tracing::info!("database snapshot written and checked");
    Ok(())
}
