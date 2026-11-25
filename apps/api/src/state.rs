use std::sync::Arc;

use diceri_application::ports::{MediaAccessIssuer, RoomIdGenerator, RoomRepository};
use diceri_application::use_cases::{JoinRoomService, RoomService};
use diceri_infra::{LiveKitAccessIssuer, MemoryRoomRepository, RandomRoomIdGenerator};

use crate::Config;

/// Shared application state injected into handlers. Cheap to clone.
#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms: RoomService,
    /// `None` when no media server is configured.
    pub join: Option<JoinRoomService>,
}

impl AppState {
    /// Composition root: wires adapters chosen by configuration.
    #[must_use]
    pub fn from_config(config: &Config) -> Self {
        let media = config
            .livekit
            .as_ref()
            .map(|livekit| -> Arc<dyn MediaAccessIssuer> {
                Arc::new(LiveKitAccessIssuer::new(
                    livekit.url.clone(),
                    livekit.api_key.clone(),
                    livekit.api_secret.clone(),
                ))
            });
        Self::in_memory(media)
    }

    /// In-memory storage with an optional media issuer (also used by tests).
    #[must_use]
    pub fn in_memory(media: Option<Arc<dyn MediaAccessIssuer>>) -> Self {
        let repository: Arc<dyn RoomRepository> = Arc::new(MemoryRoomRepository::default());
        let ids: Arc<dyn RoomIdGenerator> = Arc::new(RandomRoomIdGenerator);
        let join =
            media.map(|issuer| JoinRoomService::new(repository.clone(), ids.clone(), issuer));
        Self {
            rooms: RoomService::new(repository, ids),
            join,
        }
    }
}
