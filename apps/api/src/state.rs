use std::sync::Arc;

use diceri_application::ports::{MediaAccessIssuer, RandomSource, RoomRepository};
use diceri_application::use_cases::{AdmissionService, RoomService};
use diceri_infra::{LiveKitAccessIssuer, MemoryRoomRepository, OsRandom};

use crate::Config;

/// Shared application state injected into handlers. Cheap to clone.
#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms: RoomService,
    pub admission: AdmissionService,
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
        let random: Arc<dyn RandomSource> = Arc::new(OsRandom);
        Self {
            rooms: RoomService::new(repository.clone(), random.clone()),
            admission: AdmissionService::new(repository, random, media),
        }
    }
}
