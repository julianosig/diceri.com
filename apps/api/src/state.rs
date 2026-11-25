use std::sync::Arc;

use diceri_application::use_cases::RoomService;
use diceri_infra::{MemoryRoomRepository, RandomRoomIdGenerator};

/// Shared application state injected into handlers. Cheap to clone.
#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms: RoomService,
}

impl AppState {
    /// Composition root for local development and tests.
    #[must_use]
    pub fn in_memory() -> Self {
        let rooms = RoomService::new(
            Arc::new(MemoryRoomRepository::default()),
            Arc::new(RandomRoomIdGenerator),
        );
        Self { rooms }
    }
}
