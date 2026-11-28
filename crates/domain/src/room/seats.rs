//! Who holds a place in a call. Each account keeps one seat (its media identity), so a
//! reload replaces the old connection instead of counting twice, and a call never holds
//! more people than its size.

use crate::AccountId;

/// A seat given credentials this recently still counts while its person connects: longer
/// than those credentials (2-minute media tokens) can be used, with a margin for the time
/// between reserving the seat and signing the token, so a late connection never finds its
/// place taken by someone else.
pub(crate) const SEAT_GRACE_MS: u64 = 150_000;
/// Seats remembered per room; beyond it, the oldest seat nobody uses is forgotten.
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
pub(crate) struct Seats {
    seats: Vec<Seat>,
    /// The room's owner always has a place: guests never fill their own call before them.
    owner: Option<AccountId>,
}

impl Seats {
    pub(crate) fn reserve_for_owner(&mut self, owner: AccountId) {
        self.owner = Some(owner);
    }

    fn is_active(seat: &Seat, present: &[String], now_ms: u64) -> bool {
        present.contains(&seat.identity) || now_ms.saturating_sub(seat.issued_at_ms) < SEAT_GRACE_MS
    }

    pub(crate) fn identity_of(&self, account: &AccountId) -> Option<&str> {
        self.seats
            .iter()
            .find(|seat| seat.account == *account)
            .map(|seat| seat.identity.as_str())
    }

    /// Whether `account` can be in a call of `size`: everyone else connected or about to
    /// connect (credentials issued within `SEAT_GRACE_MS`) leaves a place free. When it is not
    /// known who is connected, the recent credentials (and the owner's place) still count, so
    /// an outage neither locks people out nor lets guests fill the owner's call.
    pub(crate) fn has_room_for(
        &self,
        account: &AccountId,
        size: u8,
        present: Option<&[String]>,
        now_ms: u64,
    ) -> bool {
        let present = present.unwrap_or_default();
        let others = self
            .seats
            .iter()
            .filter(|seat| seat.account != *account && Self::is_active(seat, present, now_ms))
            .count();
        // Someone connected whose seat was forgotten still takes a place.
        let unseated = present
            .iter()
            .filter(|identity| !self.seats.iter().any(|seat| &seat.identity == *identity))
            .count();
        // A guest leaves the owner's place free while the owner is not in.
        let owner_away = self.owner.is_some_and(|owner| {
            owner != *account
                && !self
                    .seats
                    .iter()
                    .any(|seat| seat.account == owner && Self::is_active(seat, present, now_ms))
        });
        others + unseated + usize::from(owner_away) < usize::from(size)
    }

    /// Past `MAX_SEATS`, forgets the oldest seat that is neither connected nor about to be:
    /// someone still in the call keeps theirs, so their reload is never taken for a newcomer.
    /// Without knowing who is connected, nothing is forgotten.
    fn forget_one_unused(&mut self, present: Option<&[String]>, now_ms: u64) {
        let Some(present) = present else {
            return;
        };
        if self.seats.len() < MAX_SEATS {
            return;
        }
        let unused = self
            .seats
            .iter()
            .enumerate()
            .filter(|(_, seat)| !Self::is_active(seat, present, now_ms))
            .min_by_key(|(_, seat)| seat.issued_at_ms)
            .map(|(index, _)| index);
        if let Some(index) = unused {
            self.seats.remove(index);
        }
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
        if let Some(seat) = self.seats.iter_mut().find(|seat| seat.account == account) {
            seat.issued_at_ms = now_ms;
            return Some(seat.identity.clone());
        }
        self.forget_one_unused(present, now_ms);
        self.seats.push(Seat {
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
    fn guests_never_fill_the_call_before_its_owner_is_in() {
        let mut seats = Seats::default();
        seats.reserve_for_owner(account(0));
        let first = seats.claim(account(1), "p-a".to_owned(), 2, Some(&[]), 0);
        let second = seats.claim(account(2), "p-b".to_owned(), 2, Some(&[]), 0);
        assert!(first.is_some());
        assert_eq!(second, None);
        // The owner still gets in.
        assert!(
            seats
                .claim(account(0), "p-owner".to_owned(), 2, Some(&[]), 0)
                .is_some()
        );
    }

    #[test]
    fn without_presence_only_recent_credentials_count() {
        let mut seats = Seats::default();
        seats.reserve_for_owner(account(0));
        assert!(
            seats
                .claim(account(1), "p-a".to_owned(), 2, None, 0)
                .is_some()
        );
        // The owner's place and the first guest's fresh credentials fill a call of two.
        assert_eq!(seats.claim(account(2), "p-b".to_owned(), 2, None, 0), None);
        assert!(
            seats
                .claim(account(0), "p-owner".to_owned(), 2, None, 0)
                .is_some()
        );
        // Long after, nobody is known to be in: people are not locked out.
        assert!(
            seats
                .claim(account(3), "p-c".to_owned(), 2, None, LATER)
                .is_some()
        );
    }

    #[test]
    fn a_connected_seat_is_never_forgotten() {
        let mut seats = Seats::default();
        seats.claim(account(0), "p-host".to_owned(), 2, Some(&[]), 0);
        // Twenty guests come and go through the other place while the host stays.
        for n in 1..=MAX_SEATS {
            let seed = u8::try_from(n).unwrap();
            let at = u64::try_from(n).unwrap() * LATER;
            seats.claim(
                account(seed),
                format!("p-{n}"),
                2,
                Some(&present(&["p-host"])),
                at,
            );
        }
        assert_eq!(seats.identity_of(&account(0)), Some("p-host"));
    }

    #[test]
    fn the_least_recently_used_seat_is_forgotten_but_only_when_presence_is_known() {
        let mut seats = Seats::default();
        for n in 0..=MAX_SEATS {
            let seed = u8::try_from(n).unwrap();
            let at = u64::try_from(n).unwrap() * LATER;
            seats.claim(account(seed), format!("p-{n}"), 2, Some(&[]), at);
        }
        assert_eq!(seats.identity_of(&account(0)), None);
        assert!(seats.identity_of(&account(1)).is_some());
        // Without knowing who is connected, nothing more is forgotten.
        seats.claim(account(99), "p-99".to_owned(), 2, None, LATER * 99);
        assert!(seats.identity_of(&account(1)).is_some());
    }
}
