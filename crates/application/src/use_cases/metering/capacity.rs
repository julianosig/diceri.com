//! The service's capacity: how many calls are live or about to be, counted by owner.

use std::collections::{HashMap, HashSet};

use diceri_domain::{RoomId, Username};

/// An owner's place in the service's capacity is held while their join credentials (2-minute
/// media tokens) can still be used, with a margin.
pub(super) const RESERVATION_MS: u64 = 150_000;

/// Calls counted against the service's capacity, by owner: those someone was in at the last
/// tick, and owners let in whose join credentials are still usable (reserved until then).
#[derive(Debug, Default)]
pub(super) struct LiveCalls {
    pub(super) in_call: HashSet<RoomId>,
    pub(super) live_owners: HashSet<Username>,
    /// When each owner was let in (Unix ms).
    pub(super) admitted: HashMap<Username, u64>,
}

impl LiveCalls {
    /// Distinct owners with a call live or about to be: one call never counts twice.
    /// What the minute saw. Reservations last until their credentials expire (see
    /// `MeteringService::admit`), even once the call shows up live: a fresh token stays usable
    /// if that connection drops. One for a call that could not be seen is held until a look
    /// succeeds: its person may have just connected.
    pub(super) fn update(
        &mut self,
        in_call: HashSet<RoomId>,
        live_owners: HashSet<Username>,
        unseen_owners: &HashSet<Username>,
        now_ms: u64,
    ) {
        for (owner, at) in &mut self.admitted {
            if unseen_owners.contains(owner) {
                *at = now_ms;
            }
        }
        self.in_call = in_call;
        self.live_owners = live_owners;
    }

    pub(super) fn live_count(&self) -> usize {
        let reserved = self
            .admitted
            .keys()
            .filter(|owner| !self.live_owners.contains(*owner))
            .count();
        self.live_owners.len() + reserved
    }
}
