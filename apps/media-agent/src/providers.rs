use std::sync::Arc;

use anyhow::Context as _;
use diceri_application::ports::{SpeechToText, TextToSpeech, Translator, VoiceCloner};
use diceri_infra::LiveKitAccessIssuer;
use diceri_infra::speech::{ScribeConfig, ScribeSpeechToText, SonioxConfig, SonioxSpeechToText};
use diceri_infra::synthesis::{
    ElevenLabsConfig, ElevenLabsTextToSpeech, ElevenLabsVoiceCloner, InworldConfig,
    InworldTextToSpeech, InworldVoiceCloner, TtsProvider,
};
use diceri_infra::translation::{GeminiConfig, GeminiTranslator};

use crate::config::{AgentConfig, SttProvider};

/// Everything a room session needs; built once at startup (composition root).
pub(crate) struct Providers {
    pub livekit_url: String,
    pub issuer: LiveKitAccessIssuer,
    pub stt: Arc<dyn SpeechToText>,
    pub translator: Arc<dyn Translator>,
    pub tts: Arc<dyn TextToSpeech>,
    /// Deletes participants' voice clones when they leave (ADR 0006).
    pub cloner: Arc<dyn VoiceCloner>,
}

impl Providers {
    pub(crate) fn from_config(config: &AgentConfig) -> anyhow::Result<Self> {
        let http = reqwest::Client::new();
        let (tts, cloner) = voices(config, &http)?;
        Ok(Self {
            livekit_url: config.livekit_url.clone(),
            issuer: LiveKitAccessIssuer::new(
                config.livekit_url.clone(),
                config.livekit_api_key.clone(),
                config.livekit_api_secret.clone(),
            ),
            stt: speech_to_text(config)?,
            translator: Arc::new(GeminiTranslator::new(
                GeminiConfig::new(config.gemini_api_key.clone()),
                http.clone(),
            )),
            tts,
            cloner,
        })
    }
}

/// The configured provider's synthesizer and voice cloner (the API clones with the same).
fn voices(
    config: &AgentConfig,
    http: &reqwest::Client,
) -> anyhow::Result<(Arc<dyn TextToSpeech>, Arc<dyn VoiceCloner>)> {
    Ok(match config.tts_provider {
        TtsProvider::ElevenLabs => (
            Arc::new(ElevenLabsTextToSpeech::new(
                ElevenLabsConfig::new(config.elevenlabs_api_key.clone()),
                http.clone(),
            )),
            Arc::new(ElevenLabsVoiceCloner::new(
                config.elevenlabs_api_key.clone(),
                http.clone(),
            )),
        ),
        TtsProvider::Inworld => {
            let key = config
                .inworld_api_key
                .clone()
                .context("DICERI_INWORLD_API_KEY must be set")?;
            (
                Arc::new(InworldTextToSpeech::new(
                    InworldConfig::new(key.clone()),
                    http.clone(),
                )),
                Arc::new(InworldVoiceCloner::new(key, http.clone())),
            )
        }
    })
}

fn speech_to_text(config: &AgentConfig) -> anyhow::Result<Arc<dyn SpeechToText>> {
    Ok(match config.stt_provider {
        SttProvider::Scribe => Arc::new(ScribeSpeechToText::new(ScribeConfig::new(
            config.elevenlabs_api_key.clone(),
        ))),
        SttProvider::Soniox => {
            let key = config
                .soniox_api_key
                .clone()
                .context("DICERI_SONIOX_API_KEY must be set")?;
            Arc::new(SonioxSpeechToText::new(SonioxConfig::new(key)))
        }
    })
}
