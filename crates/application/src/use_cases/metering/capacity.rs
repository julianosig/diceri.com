//! The service's capacity: how many calls are live or about to be, counted by owner.

use std::collections::{HashMap, HashSet};

use diceri_domain::{RoomId, Username};

use crate::ports::RoomRepository;

/// An owner's place in the service's capacity is held at least this long after they are
/// let in: their join credentials (2-minute media tokens) can be used until then.
pub(super) const RESERVATION_MS: u64 = 150_000;

/// Calls counted against the service's capacity, by owner: those someone was in at the last
/// look, and owners let in whose place is still held.
#[derive(Debug, Default)]
pub(super) struct LiveCalls {
    pub(super) in_call: HashSet<RoomId>,
    pub(super) live_owners: HashSet<Username>,
    /// When each owner was let in (Unix ms).
    pub(super) admitted: HashMap<Username, u64>,
    /// When the media server last answered about each owner's call (Unix ms).
    observed: HashMap<Username, u64>,
}

impl LiveCalls {
    /// What the minute saw: the calls someone is in, and the owners whose call the media
    /// server answered about (`seen_owners`); an owner it could not answer about keeps
    /// their last answer.
    pub(super) fn update(
        &mut self,
        in_call: HashSet<RoomId>,
        live_owners: HashSet<Username>,
        seen_owners: &HashSet<Username>,
        now_ms: u64,
    ) {
        for owner in seen_owners {
            self.observed.insert(owner.clone(), now_ms);
        }
        self.in_call = in_call;
        self.live_owners = live_owners;
    }

    /// Frees places no longer held. A place is held until its credentials expire and then,
    /// until the media server has answered about that call after they could last be used:
    /// a call that may have just connected is never given away on a guess. An owner seen
    /// live is counted as live anyway.
    pub(super) fn release_expired(&mut self, now_ms: u64) {
        let observed = &self.observed;
        self.admitted.retain(|owner, at| {
            let expired = now_ms.saturating_sub(*at) >= RESERVATION_MS;
            let answered_since = observed
                .get(owner)
                .is_some_and(|seen| *seen >= at.saturating_add(RESERVATION_MS));
            !(expired && answered_since)
        });
    }

    /// Owners holding a reservation.
    pub(super) fn reserved(&self) -> Vec<Username> {
        self.admitted.keys().cloned().collect()
    }

    /// Distinct owners with a call live or about to be: one call never counts twice.
    pub(super) fn live_count(&self) -> usize {
        let reserved = self
            .admitted
            .keys()
            .filter(|owner| !self.live_owners.contains(*owner))
            .count();
        self.live_owners.len() + reserved
    }
}

/// The owners of `rooms`.
pub(super) async fn owners_of(
    repository: &dyn RoomRepository,
    rooms: &HashSet<RoomId>,
) -> HashSet<Username> {
    let mut owners = HashSet::new();
    for room in rooms {
        if let Ok(Some(stored)) = repository.find_by_id(room).await {
            owners.insert(stored.owner);
        }
    }
    owners
}

/// Those of `owners` with no room open any more.
pub(super) async fn without_live_room(
    repository: &dyn RoomRepository,
    owners: Vec<Username>,
) -> Vec<Username> {
    let mut gone = Vec::new();
    for owner in owners {
        if matches!(repository.find_live_by_owner(&owner).await, Ok(None)) {
            gone.push(owner);
        }
    }
    gone
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ana() -> Username {
        "ana".parse().unwrap()
    }

    #[test]
    fn a_place_is_freed_only_after_an_answer_from_after_its_credentials_expired() {
        let mut calls = LiveCalls::default();
        calls.admitted.insert(ana(), 0);
        // Expired, but nothing has been seen since the credentials could last be used.
        calls.release_expired(RESERVATION_MS + 1);
        assert_eq!(calls.live_count(), 1);
        // An answer from before the credentials ran out proves nothing.
        calls.update(
            HashSet::new(),
            HashSet::new(),
            &HashSet::from([ana()]),
            100_000,
        );
        calls.release_expired(RESERVATION_MS + 1);
        assert_eq!(calls.live_count(), 1);
        // A later answer showing the call empty frees it.
        calls.update(
            HashSet::new(),
            HashSet::new(),
            &HashSet::from([ana()]),
            RESERVATION_MS,
        );
        calls.release_expired(RESERVATION_MS + 1);
        assert_eq!(calls.live_count(), 0);
    }
}
