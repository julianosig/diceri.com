use std::sync::Arc;

use diceri_application::ports::{
    MediaAccessIssuer, RandomSource, RoomRepository, TranslationDispatcher, VoiceCloner,
};
use diceri_application::use_cases::{AdmissionService, RoomService, VoiceService};
use diceri_infra::synthesis::ElevenLabsVoiceCloner;
use diceri_infra::{
    HttpTranslationDispatcher, LiveKitAccessIssuer, MemoryRoomRepository, OsRandom,
};

use crate::Config;

/// Shared application state injected into handlers. Cheap to clone.
#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms: RoomService,
    pub admission: AdmissionService,
    /// `None` when voice cloning is not configured.
    pub voices: Option<VoiceService>,
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
        let cloner = config
            .elevenlabs_api_key
            .as_ref()
            .map(|key| -> Arc<dyn VoiceCloner> {
                Arc::new(ElevenLabsVoiceCloner::new(
                    key.clone(),
                    reqwest::Client::new(),
                ))
            });
        let mut state = Self::in_memory_with(media, cloner);
        if let Some(agent) = &config.agent {
            let dispatcher: Arc<dyn TranslationDispatcher> =
                Arc::new(HttpTranslationDispatcher::new(
                    agent.url.clone(),
                    agent.key.clone(),
                    reqwest::Client::new(),
                ));
            state.admission = state.admission.with_translation(dispatcher);
        }
        state
    }

    /// In-memory storage with an optional media issuer (also used by tests).
    #[must_use]
    pub fn in_memory(media: Option<Arc<dyn MediaAccessIssuer>>) -> Self {
        Self::in_memory_with(media, None)
    }

    /// In-memory storage with optional media issuer and voice cloner.
    #[must_use]
    pub fn in_memory_with(
        media: Option<Arc<dyn MediaAccessIssuer>>,
        cloner: Option<Arc<dyn VoiceCloner>>,
    ) -> Self {
        let repository: Arc<dyn RoomRepository> = Arc::new(MemoryRoomRepository::default());
        let random: Arc<dyn RandomSource> = Arc::new(OsRandom);
        Self {
            rooms: RoomService::new(repository.clone(), random.clone()),
            voices: cloner.map(|cloner| VoiceService::new(repository.clone(), cloner)),
            admission: AdmissionService::new(repository, random, media),
        }
    }
}
