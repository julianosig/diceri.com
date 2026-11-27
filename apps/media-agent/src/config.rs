use std::env;
use std::net::SocketAddr;

use anyhow::Context as _;
use secrecy::SecretString;

/// Which speech-to-text provider transcribes speakers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SttProvider {
    /// ElevenLabs Scribe v3 Live: default, best balance in Nov 2026 benchmarks (ADR 0005).
    Scribe,
    Soniox,
}

/// Agent configuration from environment variables (see `.env.example`).
#[derive(Debug, Clone)]
pub(crate) struct AgentConfig {
    pub(crate) bind_address: SocketAddr,
    /// Shared secret the API sends in `x-diceri-agent-key`.
    pub(crate) control_key: SecretString,
    pub(crate) livekit_url: String,
    pub(crate) livekit_api_key: String,
    pub(crate) livekit_api_secret: SecretString,
    pub(crate) stt_provider: SttProvider,
    /// Required only when `stt_provider` is Soniox.
    pub(crate) soniox_api_key: Option<SecretString>,
    pub(crate) gemini_api_key: SecretString,
    pub(crate) elevenlabs_api_key: SecretString,
    /// `DICERI_DEPLOYMENT` (default `dev`): whose orphaned voice clones to sweep.
    pub(crate) deployment: String,
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
        Ok(Self {
            bind_address,
            control_key: secret("DICERI_AGENT_KEY")?,
            livekit_url: required("DICERI_LIVEKIT_URL")?,
            livekit_api_key: required("DICERI_LIVEKIT_API_KEY")?,
            livekit_api_secret: secret("DICERI_LIVEKIT_API_SECRET")?,
            stt_provider: stt_provider()?,
            soniox_api_key: env::var("DICERI_SONIOX_API_KEY")
                .ok()
                .map(SecretString::from),
            gemini_api_key: secret("DICERI_GEMINI_API_KEY")?,
            elevenlabs_api_key: secret("DICERI_ELEVENLABS_API_KEY")?,
            deployment: env::var("DICERI_DEPLOYMENT").unwrap_or_else(|_| "dev".to_owned()),
        })
    }
}

fn stt_provider() -> anyhow::Result<SttProvider> {
    match env::var("DICERI_STT_PROVIDER").as_deref() {
        Err(_) | Ok("scribe") => Ok(SttProvider::Scribe),
        Ok("soniox") => Ok(SttProvider::Soniox),
        Ok(other) => anyhow::bail!("DICERI_STT_PROVIDER must be scribe or soniox, got {other}"),
    }
}
