use async_trait::async_trait;
use diceri_domain::{Room, RoomCode, RoomId, RoomRuleError, Username};

#[derive(Debug, thiserror::Error)]
pub enum RoomRepositoryError {
    #[error("room `{0}` already exists")]
    AlreadyExists(RoomId),
    #[error("room `{0}` does not exist")]
    Missing(RoomId),
    /// The owner already has a room that has not ended (one live room per account).
    #[error("the owner already has a live room")]
    OwnerHasLiveRoom,
    #[error("room storage is unavailable: {0}")]
    Unavailable(String),
}

/// A change to one room, applied by `RoomRepository::modify` with no other write in between.
pub type RoomChange<'a> = Box<dyn FnOnce(&mut Room) -> Result<(), RoomRuleError> + Send + 'a>;

/// Why a `RoomChange` was not stored: the room's rules refused it, or storage failed.
#[derive(Debug, thiserror::Error)]
pub enum RoomChangeError {
    #[error(transparent)]
    Rule(#[from] RoomRuleError),
    #[error(transparent)]
    Repository(#[from] RoomRepositoryError),
}

/// Persistence for the room aggregate (including its join requests). Rooms are
/// reached by their link code from the API and by id from the media side.
#[async_trait]
pub trait RoomRepository: Send + Sync {
    /// Stores a new room; refused with `OwnerHasLiveRoom` when its owner already has a live one,
    /// checked atomically so two concurrent opens cannot both create a room.
    async fn insert(&self, room: Room) -> Result<(), RoomRepositoryError>;
    async fn find_by_code(&self, code: &RoomCode) -> Result<Option<Room>, RoomRepositoryError>;
    async fn find_by_id(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError>;
    /// The owner's room that has not ended, if any (one live room per account).
    async fn find_live_by_owner(
        &self,
        owner: &Username,
    ) -> Result<Option<Room>, RoomRepositoryError>;
    /// Applies `change` to the stored room atomically and returns the room as stored after
    /// it. Use it whenever a change depends on the room's current state: a copy read before a
    /// slow call (a provider, the database) would otherwise overwrite writes made meanwhile.
    /// Nothing is stored when `change` fails.
    async fn modify(&self, id: &RoomId, change: RoomChange<'_>) -> Result<Room, RoomChangeError>;
    /// Every room that has not ended, for the per-minute usage meter.
    async fn live_rooms(&self) -> Result<Vec<Room>, RoomRepositoryError>;
}
