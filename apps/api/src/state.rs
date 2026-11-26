use std::sync::Arc;

use diceri_application::ports::{
    MediaAccessIssuer, MediaPresence, RandomSource, RoomRepository, TranslationDispatcher,
    VoiceCloner,
};
use diceri_application::use_cases::{AdmissionService, PresenceService, RoomService, VoiceService};
use diceri_infra::synthesis::ElevenLabsVoiceCloner;
use diceri_infra::{
    HttpTranslationDispatcher, LiveKitAccessIssuer, LiveKitPresence, LiveKitRoomCloser,
    MemoryRoomRepository, OsRandom,
};

use crate::Config;

/// Shared application state injected into handlers. Cheap to clone.
#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms: RoomService,
    pub admission: AdmissionService,
    /// `None` when voice cloning is not configured.
    pub voices: Option<VoiceService>,
    /// Who is online now; empty when media is not configured.
    pub presence: PresenceService,
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
        if let Some(livekit) = &config.livekit {
            let room_closer = Arc::new(LiveKitRoomCloser::new(
                &livekit.url,
                livekit.api_key.clone(),
                livekit.api_secret.clone(),
                reqwest::Client::new(),
            ));
            state.admission = state.admission.with_room_closer(room_closer);
            let presence: Arc<dyn MediaPresence> = Arc::new(LiveKitPresence::new(
                &livekit.url,
                livekit.api_key.clone(),
                livekit.api_secret.clone(),
                reqwest::Client::new(),
            ));
            state.presence = PresenceService::new(Some(presence));
        }
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
            presence: PresenceService::new(None),
        }
    }
}
