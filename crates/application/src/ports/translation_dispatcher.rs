use async_trait::async_trait;
use diceri_domain::RoomId;

#[derive(Debug, thiserror::Error)]
#[error("failed to start translation for the room: {0}")]
pub struct DispatchError(pub String);

/// Asks the media agent to join a room and translate it. Idempotent: calling it
/// for a room that is already being translated is a no-op.
#[async_trait]
pub trait TranslationDispatcher: Send + Sync {
    async fn start(&self, room: &RoomId) -> Result<(), DispatchError>;
}
