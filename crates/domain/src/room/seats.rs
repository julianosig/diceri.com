//! Who holds a place in a call. Each account keeps one seat (its media identity), so a
//! reload replaces the old connection instead of counting twice, and a call never holds
//! more people than its size.

use crate::AccountId;

/// A seat given credentials this recently still counts while its person connects: as long
/// as those credentials (media tokens) can be used, so a late connection never finds its
/// place taken by someone else.
pub(crate) const SEAT_GRACE_MS: u64 = 120_000;
/// Seats remembered per room; the least recently used is forgotten.
pub(crate) const MAX_SEATS: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Seat {
    account: AccountId,
    identity: String,
    /// When credentials were last issued for this seat (Unix ms).
    issued_at_ms: u64,
}

/// A room's seats; `present` lists the identities connected to the media server now, or
/// `None` when it cannot say (then size is not enforced: an outage must not lock people out).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Seats(Vec<Seat>);

impl Seats {
    pub(crate) fn identity_of(&self, account: &AccountId) -> Option<&str> {
        self.0
            .iter()
            .find(|seat| seat.account == *account)
            .map(|seat| seat.identity.as_str())
    }

    /// Whether `account` can be in a call of `size`: everyone else connected or about to
    /// connect (credentials issued within `SEAT_GRACE_MS`) leaves a place free.
    pub(crate) fn has_room_for(
        &self,
        account: &AccountId,
        size: u8,
        present: Option<&[String]>,
        now_ms: u64,
    ) -> bool {
        let Some(present) = present else {
            return true;
        };
        let others = self
            .0
            .iter()
            .filter(|seat| seat.account != *account)
            .filter(|seat| {
                present.contains(&seat.identity)
                    || now_ms.saturating_sub(seat.issued_at_ms) < SEAT_GRACE_MS
            })
            .count();
        // Someone connected whose seat was forgotten still takes a place.
        let unseated = present
            .iter()
            .filter(|identity| !self.0.iter().any(|seat| &seat.identity == *identity))
            .count();
        others + unseated < usize::from(size)
    }

    /// `account`'s seat, refreshed (or a new one with `identity`), if the call has room.
    pub(crate) fn claim(
        &mut self,
        account: AccountId,
        identity: String,
        size: u8,
        present: Option<&[String]>,
        now_ms: u64,
    ) -> Option<String> {
        if !self.has_room_for(&account, size, present, now_ms) {
            return None;
        }
        if let Some(seat) = self.0.iter_mut().find(|seat| seat.account == account) {
            seat.issued_at_ms = now_ms;
            return Some(seat.identity.clone());
        }
        if self.0.len() >= MAX_SEATS
            && let Some((oldest, _)) = self
                .0
                .iter()
                .enumerate()
                .min_by_key(|(_, seat)| seat.issued_at_ms)
        {
            self.0.remove(oldest);
        }
        self.0.push(Seat {
            account,
            identity: identity.clone(),
            issued_at_ms: now_ms,
        });
        Some(identity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LATER: u64 = SEAT_GRACE_MS * 10;

    fn account(seed: u8) -> AccountId {
        AccountId::from_parts(1, [seed; 10])
    }

    fn present(identities: &[&str]) -> Vec<String> {
        identities.iter().map(|&id| id.to_owned()).collect()
    }

    #[test]
    fn a_reload_reuses_the_seat_and_replaces_the_old_connection() {
        let mut seats = Seats::default();
        let ana = seats.claim(account(1), "p-ana".to_owned(), 2, Some(&[]), 0);
        seats.claim(account(2), "p-bob".to_owned(), 2, Some(&[]), 0);
        // Both connected, the call is full, and Ana reloads: her old tab is her own seat.
        let now = present(&["p-ana", "p-bob"]);
        let again = seats.claim(account(1), "p-new".to_owned(), 2, Some(&now), LATER);
        assert_eq!(again, ana);
    }

    #[test]
    fn two_people_let_in_at_once_cannot_both_take_the_last_place() {
        let mut seats = Seats::default();
        seats.claim(account(1), "p-host".to_owned(), 2, Some(&[]), 0);
        let host_only = present(&["p-host"]);
        // Neither guest has connected yet, but the first one's fresh credentials count.
        let first = seats.claim(account(2), "p-a".to_owned(), 2, Some(&host_only), 1_000);
        let second = seats.claim(account(3), "p-b".to_owned(), 2, Some(&host_only), 1_000);
        assert_eq!(first.as_deref(), Some("p-a"));
        assert_eq!(second, None);
    }

    #[test]
    fn someone_who_left_returns_only_if_their_place_is_still_free() {
        let mut seats = Seats::default();
        seats.claim(account(1), "p-host".to_owned(), 2, Some(&[]), 0);
        seats.claim(account(2), "p-bob".to_owned(), 2, Some(&[]), 0);
        // Bob left and Carla took the place; Bob's remembered seat does not get him back in.
        let now = present(&["p-host"]);
        seats.claim(account(3), "p-carla".to_owned(), 2, Some(&now), LATER);
        let full = present(&["p-host", "p-carla"]);
        assert_eq!(
            seats.claim(account(2), "p-x".to_owned(), 2, Some(&full), LATER),
            None
        );
    }

    #[test]
    fn without_presence_size_is_not_enforced() {
        let mut seats = Seats::default();
        for seed in 1..=5 {
            assert!(
                seats
                    .claim(account(seed), format!("p-{seed}"), 2, None, 0)
                    .is_some()
            );
        }
    }

    #[test]
    fn the_least_recently_used_seat_is_forgotten() {
        let mut seats = Seats::default();
        for n in 0..=MAX_SEATS {
            let seed = u8::try_from(n).unwrap();
            let at = u64::try_from(n).unwrap() * LATER;
            seats.claim(account(seed), format!("p-{n}"), 2, None, at);
        }
        assert_eq!(seats.identity_of(&account(0)), None);
        assert!(seats.identity_of(&account(1)).is_some());
    }
}
