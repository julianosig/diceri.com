//! What each plan allows, the capacity shared by everyone, and the limits a call can hit.

use diceri_domain::{Plan, PlanLimits, Room};

use crate::ports::{
    AccountRepositoryError, RoomChangeError, RoomRepositoryError, UsageRepositoryError,
};

/// What each plan allows. Free is everyone's default; the others are assigned by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanCatalogue {
    pub free: PlanLimits,
    pub pro: PlanLimits,
    pub team: PlanLimits,
}

impl PlanCatalogue {
    #[must_use]
    pub fn limits(&self, plan: Plan) -> PlanLimits {
        match plan {
            Plan::Free => self.free,
            Plan::Pro => self.pro,
            Plan::Team => self.team,
        }
    }
}

impl Default for PlanCatalogue {
    fn default() -> Self {
        // Priced to keep a margin at full use; see docs/pricing.md.
        let limits =
            |call_minutes, daily_minutes, monthly_minutes, monthly_price_cents| PlanLimits {
                call_minutes,
                daily_minutes,
                monthly_minutes,
                monthly_price_cents,
                participants: Room::DEFAULT_MAX_PARTICIPANTS,
            };
        Self {
            // 60 minutes a month, usable in one call or spread over days.
            free: limits(60, 60, 60, 0),
            pro: limits(60, 300, 300, 2_900),
            team: limits(120, 600, 800, 7_900),
        }
    }
}

/// Limits across every account, so the shared free capacity is never exhausted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capacity {
    /// Calls with someone in them at once, across the whole service.
    pub max_live_calls: usize,
    /// Call minutes per calendar month (UTC), across every account.
    pub monthly_minutes: u64,
}

impl Default for Capacity {
    fn default() -> Self {
        Self {
            max_live_calls: 15,
            monthly_minutes: 1_500,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LimitError {
    #[error("today's call minutes are used up")]
    DailyMinutes,
    #[error("this month's call minutes on the plan are used up")]
    PlanMonthly,
    #[error("this month's free capacity is used up")]
    MonthlyCapacity,
    #[error("too many calls are live right now")]
    Busy,
    #[error(transparent)]
    Usage(#[from] UsageRepositoryError),
    #[error(transparent)]
    Accounts(#[from] AccountRepositoryError),
    #[error(transparent)]
    Rooms(#[from] RoomRepositoryError),
    /// A room's rules refused a change made while metering.
    #[error(transparent)]
    RoomRule(#[from] diceri_domain::RoomRuleError),
}

impl From<RoomChangeError> for LimitError {
    fn from(error: RoomChangeError) -> Self {
        match error {
            RoomChangeError::Rule(rule) => Self::RoomRule(rule),
            RoomChangeError::Repository(repository) => Self::Rooms(repository),
        }
    }
}

/// An account's plan, its limits and what it has used today and this month.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Allowance {
    pub plan: Plan,
    pub limits: PlanLimits,
    pub used_today: u32,
    pub used_this_month: u32,
}

impl Allowance {
    /// The first limit already reached, if any (checked before a call starts).
    #[must_use]
    pub fn reached(&self) -> Option<LimitError> {
        if self.used_today >= self.limits.daily_minutes {
            return Some(LimitError::DailyMinutes);
        }
        (self.used_this_month >= self.limits.monthly_minutes).then_some(LimitError::PlanMonthly)
    }

    /// Minutes left today and this month, whichever runs out first.
    #[must_use]
    pub fn minutes_left(&self) -> u32 {
        let day = self.limits.daily_minutes.saturating_sub(self.used_today);
        let month = self
            .limits
            .monthly_minutes
            .saturating_sub(self.used_this_month);
        day.min(month)
    }
}
