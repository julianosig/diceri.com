use std::sync::Arc;

use anyhow::Context as _;
use diceri_application::ports::{
    AccountRepository, IdGenerator, MediaAccessIssuer, MediaPresence, RandomSource, RoomRepository,
    SpeechToText, TranslationDispatcher, VoiceCloner,
};
use diceri_application::use_cases::{
    AccountService, AdmissionService, Capacity, MeteringService, PlanCatalogue, PresenceService,
    RoomService, VoiceService,
};
use diceri_infra::speech::{SonioxConfig, SonioxSpeechToText};
use diceri_infra::synthesis::ElevenLabsVoiceCloner;
use diceri_infra::{
    Argon2Hasher, HttpTranslationDispatcher, LiveKitAccessIssuer, LiveKitPresence,
    LiveKitRoomCloser, MemoryAccountRepository, MemoryRoomRepository, MemoryUsageRepository,
    MemoryVoiceSamples, OsRandom, SqliteAccountRepository, SqliteUsageRepository,
    SqliteVoiceSamples,
};

use crate::Config;
use crate::features::SignupNetworks;

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
    /// Plans, call-minute metering and the limits that keep free capacity fair.
    pub metering: MeteringService,
    /// Session cookies are `Secure` (HTTPS only).
    pub secure_cookies: bool,
    /// One account per network; `None` when not enforced (no key configured).
    pub(crate) signup_networks: Option<SignupNetworks>,
    /// Joining needs a recorded voice (only when voice cloning is configured).
    pub voice_required: bool,
}

impl AppState {
    /// Allows `max_accounts` sign-ups per network, hashing addresses with `key`;
    /// `ip_header` names the proxy header with the real client address.
    #[must_use]
    pub fn limit_accounts_per_network(
        mut self,
        key: secrecy::SecretString,
        max_accounts: u32,
        ip_header: Option<axum::http::HeaderName>,
    ) -> Self {
        self.signup_networks = Some(SignupNetworks::new(key, max_accounts, ip_header));
        self
    }

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
        // Voice cloning is off unless switched on: no cloner, no voice service, nothing sent.
        let cloner = config
            .elevenlabs_api_key
            .as_ref()
            .filter(|_| config.voice_cloning)
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
        let sqlite = SqliteAccountRepository::connect(&config.database_url)
            .await
            .context("cannot open the accounts database")?;
        let usage = Arc::new(SqliteUsageRepository::sharing(&sqlite));
        state.voices = keep_recordings(state.voices, &sqlite, config);
        let accounts: Arc<dyn AccountRepository> = Arc::new(sqlite);
        state.accounts = account_service(accounts.clone());
        state.metering = state
            .metering
            .with_stores(accounts, usage)
            .with_limits(config.plans, config.capacity);
        state.secure_cookies = config.secure_cookies;
        state.voice_required = config.voice_required && state.voices.is_some();
        if let Some(key) = config.network_key.clone() {
            let header = config
                .client_ip_header
                .as_deref()
                .and_then(|name| name.parse().ok());
            state = state.limit_accounts_per_network(key, config.accounts_per_network, header);
        } else {
            tracing::warn!("DICERI_NETWORK_KEY unset: accounts per network are not limited");
        }
        if let Some(livekit) = &config.livekit {
            let room_closer = Arc::new(LiveKitRoomCloser::new(
                &livekit.url,
                livekit.api_key.clone(),
                livekit.api_secret.clone(),
                reqwest::Client::new(),
            ));
            state.admission = state.admission.with_room_closer(room_closer.clone());
            let presence: Arc<dyn MediaPresence> = Arc::new(LiveKitPresence::new(
                &livekit.url,
                livekit.api_key.clone(),
                livekit.api_secret.clone(),
                reqwest::Client::new(),
            ));
            state.presence = state.presence.with_source(presence.clone());
            state.metering = state.metering.with_media(presence, room_closer);
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
        let accounts: Arc<dyn AccountRepository> = Arc::new(MemoryAccountRepository::default());
        Self {
            rooms: RoomService::new(repository.clone(), random.clone(), ids.clone()),
            voices: cloner.zip(recognizer).map(|(cloner, recognizer)| {
                VoiceService::new(repository.clone(), cloner, recognizer, deployment)
                    .with_samples(Arc::new(MemoryVoiceSamples::default()), Arc::new(OsRandom))
            }),
            presence: PresenceService::new(None, repository.clone()),
            accounts: account_service(accounts.clone()),
            metering: MeteringService::new(
                repository.clone(),
                accounts,
                Arc::new(MemoryUsageRepository::default()),
                Arc::new(OsRandom),
                PlanCatalogue::default(),
                Capacity::default(),
            ),
            secure_cookies: false,
            signup_networks: None,
            voice_required: false,
            admission: AdmissionService::new(repository, random, ids, media),
        }
    }
}

/// Keeps each account's recording encrypted in the accounts database (ADR 0015) when a
/// voice key is configured; without one, recordings are used for the call and dropped.
fn keep_recordings(
    voices: Option<VoiceService>,
    sqlite: &SqliteAccountRepository,
    config: &Config,
) -> Option<VoiceService> {
    use secrecy::ExposeSecret as _;
    let voices = voices?;
    let Some(key) = &config.voice_key else {
        tracing::warn!("DICERI_VOICE_KEY unset: voice recordings are not kept");
        return Some(voices.without_samples());
    };
    let samples = Arc::new(SqliteVoiceSamples::sharing(sqlite, *key.expose_secret()));
    Some(voices.with_samples(samples, Arc::new(OsRandom)))
}
