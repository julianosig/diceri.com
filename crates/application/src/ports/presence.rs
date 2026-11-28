use diceri_domain::{LanguageCode, RoomId};

use super::MediaAccessError;

/// A media room as seen right now: how many people are in and what they listen in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveRoom {
    pub room: RoomId,
    pub people: usize,
    /// Output language of each person that has one (duplicates kept).
    pub languages: Vec<LanguageCode>,
}

/// Media identities in one call: the people, and the hidden listeners (`monitor-<seat>`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CallMembers {
    pub people: Vec<String>,
    pub listeners: Vec<String>,
}

/// Who is connected to the media server at this moment.
#[async_trait::async_trait]
pub trait MediaPresence: Send + Sync {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError>;

    /// Every media room open now, by room id (one cheap listing, no participants and no cap),
    /// so calls that ended can be closed.
    async fn open_rooms(&self) -> Result<Vec<RoomId>, MediaAccessError>;

    /// Media identities of the people in one room's call right now (empty when it does not
    /// exist), so a call can be kept to its size.
    async fn identities_in(&self, room: &RoomId) -> Result<Vec<String>, MediaAccessError>;

    /// People and hidden listeners (loopback monitors) in one room's call, from a single
    /// look, so a listener is never judged against an older list of people.
    async fn call_members(&self, room: &RoomId) -> Result<CallMembers, MediaAccessError>;

    /// People in one room's call right now (0 when the media room does not exist).
    /// Adapters should answer with a single-room query; this default scans every room.
    async fn people_in(&self, room: &RoomId) -> Result<usize, MediaAccessError> {
        let rooms = self.live_rooms().await?;
        Ok(rooms
            .into_iter()
            .find(|live| live.room == *room)
            .map_or(0, |live| live.people))
    }
}

/// What the media server reports about a call as it happens (LiveKit webhooks, ADR 0018).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaEvent {
    /// Someone connected to `room`'s call (also after a reconnect).
    Joined { room: RoomId, identity: String },
    /// Someone left `room`'s call.
    Left { room: RoomId, identity: String },
    /// The media room closed.
    Finished { room: RoomId },
}

#[derive(Debug, thiserror::Error)]
pub enum MediaEventError {
    /// Not signed by the media server (or signed for another body).
    #[error("media event is not signed by the media server")]
    Unsigned,
    #[error("media event could not be read")]
    Unreadable,
}

/// Checks that a report really comes from the media server and reads it.
pub trait MediaEventVerifier: Send + Sync {
    /// The event in `body`, signed in `authorization`; `None` for events not used here or
    /// rooms not named after a room id.
    fn verify(
        &self,
        body: &str,
        authorization: &str,
    ) -> Result<Option<MediaEvent>, MediaEventError>;
}
