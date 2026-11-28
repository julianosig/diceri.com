//! Plan limits and shared capacity from the environment; every value has a default, so
//! nothing needs setting to launch. See the table in `apps/api/AGENTS.md`.

use std::env;
use std::str::FromStr;

use anyhow::Context as _;
use diceri_application::use_cases::{Capacity, PlanCatalogue};
use diceri_domain::PlanLimits;

/// `name` parsed as a number, or `default` when unset.
fn number<T: FromStr>(name: &str, default: T) -> anyhow::Result<T> {
    match env::var(name) {
        Ok(raw) => raw
            .parse()
            .ok()
            .with_context(|| format!("{name} must be a whole number")),
        Err(_) => Ok(default),
    }
}

/// One plan: `DICERI_<PLAN>_{CALL,DAILY,MONTHLY}_MINUTES` and `DICERI_<PLAN>_PRICE_CENTS`.
fn plan(prefix: &str, defaults: PlanLimits) -> anyhow::Result<PlanLimits> {
    Ok(PlanLimits {
        call_minutes: number(
            &format!("DICERI_{prefix}_CALL_MINUTES"),
            defaults.call_minutes,
        )?,
        daily_minutes: number(
            &format!("DICERI_{prefix}_DAILY_MINUTES"),
            defaults.daily_minutes,
        )?,
        monthly_minutes: number(
            &format!("DICERI_{prefix}_MONTHLY_MINUTES"),
            defaults.monthly_minutes,
        )?,
        monthly_price_cents: number(
            &format!("DICERI_{prefix}_PRICE_CENTS"),
            defaults.monthly_price_cents,
        )?,
        participants: number(
            &format!("DICERI_{prefix}_PARTICIPANTS"),
            defaults.participants,
        )?,
    })
}

pub(crate) fn plans_from_env() -> anyhow::Result<PlanCatalogue> {
    let defaults = PlanCatalogue::default();
    Ok(PlanCatalogue {
        free: plan("FREE", defaults.free)?,
        pro: plan("PRO", defaults.pro)?,
        team: plan("TEAM", defaults.team)?,
    })
}

pub(crate) fn capacity_from_env() -> anyhow::Result<Capacity> {
    let defaults = Capacity::default();
    Ok(Capacity {
        max_live_calls: number("DICERI_MAX_LIVE_CALLS", defaults.max_live_calls)?,
        monthly_minutes: number("DICERI_MONTHLY_CALL_MINUTES", defaults.monthly_minutes)?,
    })
}
