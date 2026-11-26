use std::sync::Arc;

use anyhow::Context as _;
use diceri_application::ports::{
    AccountRepository, IdGenerator, MediaAccessIssuer, MediaPresence, RandomSource, RoomRepository,
    SpeechToText, TranslationDispatcher, VoiceCloner,
};
use diceri_application::use_cases::{
    AccountService, AdmissionService, PresenceService, RoomService, VoiceService,
};
use diceri_infra::speech::{SonioxConfig, SonioxSpeechToText};
use diceri_infra::synthesis::ElevenLabsVoiceCloner;
use diceri_infra::{
    Argon2Hasher, HttpTranslationDispatcher, LiveKitAccessIssuer, LiveKitPresence,
    LiveKitRoomCloser, MemoryAccountRepository, MemoryRoomRepository, OsRandom,
    SqliteAccountRepository,
};

use crate::Config;

/// Accounts over `repository` with Argon2id passwords and the OS clock/randomness.
fn account_service(repository: Arc<dyn AccountRepository>) -> AccountService {
    AccountService::new(
        repository,
        Arc::new(Argon2Hasher),
        Arc::new(OsRandom),
        Arc::new(OsRandom),
        Arc::new(OsRandom),
    )
}

/// Shared application state injected into handlers. Cheap to clone.
#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms: RoomService,
    pub admission: AdmissionService,
    /// `None` when voice cloning is not configured.
    pub voices: Option<VoiceService>,
    /// Who is online now; empty when media is not configured.
    pub presence: PresenceService,
    pub accounts: AccountService,
    /// Session cookies are `Secure` (HTTPS only).
    pub secure_cookies: bool,
}

impl AppState {
    /// Composition root: wires adapters chosen by configuration.
    pub async fn from_config(config: &Config) -> anyhow::Result<Self> {
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
        let recognizer = config
            .soniox_api_key
            .as_ref()
            .map(|key| -> Arc<dyn SpeechToText> {
                Arc::new(SonioxSpeechToText::new(SonioxConfig::new(key.clone())))
            });
        let mut state = Self::in_memory_with(media, cloner, recognizer, &config.deployment);
        let accounts: Arc<dyn AccountRepository> = Arc::new(
            SqliteAccountRepository::connect(&config.database_url)
                .await
                .context("cannot open the accounts database")?,
        );
        state.accounts = account_service(accounts);
        state.secure_cookies = config.secure_cookies;
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
            state.presence = state.presence.with_source(presence);
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
        Ok(state)
    }

    /// In-memory storage with an optional media issuer (also used by tests).
    #[must_use]
    pub fn in_memory(media: Option<Arc<dyn MediaAccessIssuer>>) -> Self {
        Self::in_memory_with(media, None, None, "test")
    }

    /// In-memory storage with optional media issuer, voice cloner and the recognizer that
    /// checks recordings are speech (voices need both); `deployment` names voice clones
    /// (see `clone_name_prefix`).
    #[must_use]
    pub fn in_memory_with(
        media: Option<Arc<dyn MediaAccessIssuer>>,
        cloner: Option<Arc<dyn VoiceCloner>>,
        recognizer: Option<Arc<dyn SpeechToText>>,
        deployment: &str,
    ) -> Self {
        let repository: Arc<dyn RoomRepository> = Arc::new(MemoryRoomRepository::default());
        let random: Arc<dyn RandomSource> = Arc::new(OsRandom);
        let ids: Arc<dyn IdGenerator> = Arc::new(OsRandom);
        Self {
            rooms: RoomService::new(repository.clone(), random.clone(), ids.clone()),
            voices: cloner.zip(recognizer).map(|(cloner, recognizer)| {
                VoiceService::new(repository.clone(), cloner, recognizer, deployment)
            }),
            presence: PresenceService::new(None, repository.clone()),
            accounts: account_service(Arc::new(MemoryAccountRepository::default())),
            secure_cookies: false,
            admission: AdmissionService::new(repository, random, ids, media),
        }
    }
}
