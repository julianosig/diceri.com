use std::env;
use std::net::SocketAddr;

use anyhow::Context as _;
use diceri_application::ports::VoiceId;
use secrecy::SecretString;

/// Stock multilingual voices used until consented voice clones exist.
const DEFAULT_VOICES: &[&str] = &[
    "EXAVITQu4vr4xnSDxMaL", // Sarah
    "CwhRBWXzGAHq8TQ4Fs17", // Roger
    "FGY2WhTYpPnrIDTdsKH5", // Laura
    "IKne3meq5aSn9XLyUdCD", // Charlie
];

/// Agent configuration from environment variables (see `.env.example`).
#[derive(Debug, Clone)]
pub(crate) struct AgentConfig {
    pub(crate) bind_address: SocketAddr,
    /// Shared secret the API sends in `x-diceri-agent-key`.
    pub(crate) control_key: SecretString,
    pub(crate) livekit_url: String,
    pub(crate) livekit_api_key: String,
    pub(crate) livekit_api_secret: SecretString,
    pub(crate) soniox_api_key: SecretString,
    pub(crate) gemini_api_key: SecretString,
    pub(crate) elevenlabs_api_key: SecretString,
    pub(crate) voices: Vec<VoiceId>,
}

fn required(name: &str) -> anyhow::Result<String> {
    env::var(name).with_context(|| format!("{name} must be set"))
}

fn secret(name: &str) -> anyhow::Result<SecretString> {
    required(name).map(SecretString::from)
}

impl AgentConfig {
    pub(crate) fn from_env() -> anyhow::Result<Self> {
        let bind_address = env::var("DICERI_AGENT_BIND_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1:8095".to_owned())
            .parse()
            .context("DICERI_AGENT_BIND_ADDRESS must be a socket address")?;
        let voices = env::var("DICERI_TTS_VOICES").map_or_else(
            |_| {
                DEFAULT_VOICES
                    .iter()
                    .map(|id| VoiceId((*id).to_owned()))
                    .collect()
            },
            |list| {
                list.split(',')
                    .map(|id| VoiceId(id.trim().to_owned()))
                    .collect()
            },
        );
        Ok(Self {
            bind_address,
            control_key: secret("DICERI_AGENT_KEY")?,
            livekit_url: required("DICERI_LIVEKIT_URL")?,
            livekit_api_key: required("DICERI_LIVEKIT_API_KEY")?,
            livekit_api_secret: secret("DICERI_LIVEKIT_API_SECRET")?,
            soniox_api_key: secret("DICERI_SONIOX_API_KEY")?,
            gemini_api_key: secret("DICERI_GEMINI_API_KEY")?,
            elevenlabs_api_key: secret("DICERI_ELEVENLABS_API_KEY")?,
            voices,
        })
    }
}
