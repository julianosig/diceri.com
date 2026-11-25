use std::sync::Arc;

use diceri_application::ports::{SpeechToText, TextToSpeech, Translator, VoiceId};
use diceri_infra::LiveKitAccessIssuer;
use diceri_infra::speech::{SonioxConfig, SonioxSpeechToText};
use diceri_infra::synthesis::{ElevenLabsConfig, ElevenLabsTextToSpeech};
use diceri_infra::translation::{GeminiConfig, GeminiTranslator};

use crate::config::AgentConfig;

/// Everything a room session needs; built once at startup (composition root).
pub(crate) struct Providers {
    pub livekit_url: String,
    pub issuer: LiveKitAccessIssuer,
    pub stt: Arc<dyn SpeechToText>,
    pub translator: Arc<dyn Translator>,
    pub tts: Arc<dyn TextToSpeech>,
    pub voices: Vec<VoiceId>,
}

impl Providers {
    pub(crate) fn from_config(config: &AgentConfig) -> Self {
        let http = reqwest::Client::new();
        Self {
            livekit_url: config.livekit_url.clone(),
            issuer: LiveKitAccessIssuer::new(
                config.livekit_url.clone(),
                config.livekit_api_key.clone(),
                config.livekit_api_secret.clone(),
            ),
            stt: Arc::new(SonioxSpeechToText::new(SonioxConfig::new(
                config.soniox_api_key.clone(),
            ))),
            translator: Arc::new(GeminiTranslator::new(
                GeminiConfig::new(config.gemini_api_key.clone()),
                http.clone(),
            )),
            tts: Arc::new(ElevenLabsTextToSpeech::new(
                ElevenLabsConfig::new(config.elevenlabs_api_key.clone()),
                http,
            )),
            voices: config.voices.clone(),
        }
    }
}
