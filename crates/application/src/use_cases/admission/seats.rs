//! Keeping calls to their size: who is in, and each account's seat.

use diceri_domain::{AccountId, Room, RoomId, RoomRuleError};

use super::{AdmissionError, AdmissionService};

/// How long joining waits to learn who is in a call before counting it as unknown.
const PRESENCE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(3);

impl AdmissionService {
    /// `account`'s place in the call (its media identity), reserved on the room as it is now:
    /// a reload reuses it, and nobody gets in once the call has ended or is full, even when
    /// two people are let in at the same moment.
    pub(super) async fn seat(
        &self,
        room: &Room,
        account: &AccountId,
    ) -> Result<String, AdmissionError> {
        let (present, now_ms) = self.who_is_in(room).await;
        let (account, identity) = (*account, format!("p-{}", self.ids.participant_id()));
        let claim = move |room: &mut Room| {
            room.claim_seat(account, identity, present.as_deref(), now_ms)
                .map(|_| ())
        };
        let seated = self.rooms.modify(&room.id, Box::new(claim)).await?;
        seated
            .seat_of(&account)
            .map(str::to_owned)
            .ok_or(AdmissionError::NotFound)
    }

    /// Refuses a newcomer to a full call up front, before they wait for the host.
    pub(super) async fn ensure_space(
        &self,
        room: &Room,
        account: &AccountId,
    ) -> Result<(), AdmissionError> {
        let (present, now_ms) = self.who_is_in(room).await;
        if room.has_room_for(account, present.as_deref(), now_ms) {
            Ok(())
        } else {
            Err(AdmissionError::Rule(RoomRuleError::Full))
        }
    }

    /// Who is connected to `room`'s call and the time now; `None` when the media server cannot
    /// say (then size is not enforced: an outage must not lock people out).
    pub(super) async fn who_is_in(&self, room: &Room) -> (Option<Vec<String>>, u64) {
        let Some((presence, clock)) = &self.presence else {
            return (None, 0);
        };
        // Read before asking: a seat whose time runs out while the answer is on its way was
        // still held when the media server listed who is in.
        let now_ms = clock.now_ms();
        // A slow media server must not hold up joining: then who is in counts as unknown.
        let answer = tokio::time::timeout(PRESENCE_TIMEOUT, presence.identities_in(&room.id)).await;
        let identities = match answer {
            Ok(Ok(identities)) => Some(identities),
            Ok(Err(error)) => warn_size_unchecked(&room.id, &error.to_string()),
            Err(_) => warn_size_unchecked(&room.id, "media server too slow"),
        };
        (identities, now_ms)
    }
}

fn warn_size_unchecked(room: &RoomId, reason: &str) -> Option<Vec<String>> {
    tracing::warn!(room_id = %room, reason, "call size not checked");
    None
}
