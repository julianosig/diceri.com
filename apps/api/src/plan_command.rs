//! `diceri-api plan <username> <free|pro|team>`: assigns an account's plan by hand, while
//! plans are not sold. Runs beside the live server; the change applies on the next request.

use anyhow::Context as _;
use diceri_application::ports::AccountRepository as _;
use diceri_domain::{Plan, Username};
use diceri_infra::SqliteAccountRepository;

use crate::Config;

pub async fn run(config: &Config, username: &str, plan: &str) -> anyhow::Result<()> {
    let username: Username = username.parse().context("not a valid username")?;
    let plan: Plan = plan.parse().context("the plan must be free, pro or team")?;
    let accounts = SqliteAccountRepository::connect(&config.database_url)
        .await
        .context("cannot open the accounts database")?;
    let found = accounts
        .set_plan(&username, plan)
        .await
        .context("could not change the plan")?;
    anyhow::ensure!(found, "no account has that username");
    // Logs only the plan, never the username (AGENTS.md §14 rule 27).
    tracing::info!(%plan, "plan changed");
    Ok(())
}
