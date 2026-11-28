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

    /// Media identities of the hidden listeners (loopback monitors) in one room's call.
    async fn listeners_in(&self, room: &RoomId) -> Result<Vec<String>, MediaAccessError>;

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
