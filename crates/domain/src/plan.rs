use std::fmt;
use std::str::FromStr;

/// What an account may use. Everyone starts on `Free`; the others are assigned by hand
/// until they are sold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Plan {
    #[default]
    Free,
    Pro,
    Team,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("unknown plan")]
pub struct PlanError;

impl Plan {
    pub const ALL: [Self; 3] = [Self::Free, Self::Pro, Self::Team];

    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Free => "free",
            Self::Pro => "pro",
            Self::Team => "team",
        }
    }
}

impl FromStr for Plan {
    type Err = PlanError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|plan| plan.as_str() == raw)
            .ok_or(PlanError)
    }
}

impl fmt::Display for Plan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// How much a plan allows. Minutes count only while someone is in the call, and are
/// charged to the room's owner (guests use their host's minutes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanLimits {
    /// Longest single call, in minutes.
    pub call_minutes: u32,
    /// Call minutes per UTC day, across all of the owner's calls.
    pub daily_minutes: u32,
    /// People in a room at once, the host included.
    pub participants: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_plan_round_trips_by_name_and_free_is_the_default() {
        for plan in Plan::ALL {
            assert_eq!(plan.as_str().parse::<Plan>(), Ok(plan));
        }
        assert_eq!(Plan::default(), Plan::Free);
        assert_eq!("gold".parse::<Plan>(), Err(PlanError));
    }
}
