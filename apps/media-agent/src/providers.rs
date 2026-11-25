use std::sync::Arc;

use anyhow::Context as _;
use diceri_application::ports::{SpeechToText, TextToSpeech, Translator, VoiceCloner, VoiceId};
use diceri_infra::LiveKitAccessIssuer;
use diceri_infra::speech::{ScribeConfig, ScribeSpeechToText, SonioxConfig, SonioxSpeechToText};
use diceri_infra::synthesis::{ElevenLabsConfig, ElevenLabsTextToSpeech, ElevenLabsVoiceCloner};
use diceri_infra::translation::{GeminiConfig, GeminiTranslator};

use crate::config::{AgentConfig, SttProvider};

/// Everything a room session needs; built once at startup (composition root).
pub(crate) struct Providers {
    pub livekit_url: String,
    pub issuer: LiveKitAccessIssuer,
    pub stt: Arc<dyn SpeechToText>,
    pub translator: Arc<dyn Translator>,
    pub tts: Arc<dyn TextToSpeech>,
    /// Stock voices for people who did not record their own.
    pub voices: Vec<VoiceId>,
    /// Deletes participants' voice clones when they leave (ADR 0006).
    pub cloner: Arc<dyn VoiceCloner>,
}

impl Providers {
    pub(crate) fn from_config(config: &AgentConfig) -> anyhow::Result<Self> {
        let http = reqwest::Client::new();
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
            tts: Arc::new(ElevenLabsTextToSpeech::new(
                ElevenLabsConfig::new(config.elevenlabs_api_key.clone()),
                http.clone(),
            )),
            cloner: Arc::new(ElevenLabsVoiceCloner::new(
                config.elevenlabs_api_key.clone(),
                http,
            )),
            voices: config.voices.clone(),
        })
    }
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
