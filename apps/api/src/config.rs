use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::Context as _;
use secrecy::SecretString;

/// Log output format, selected with `DICERI_LOG_FORMAT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Pretty,
    Json,
}

/// LiveKit project credentials. All three variables must be set together.
#[derive(Debug, Clone)]
pub struct LiveKitConfig {
    pub url: String,
    pub api_key: String,
    pub api_secret: SecretString,
}

/// Where the media agent's control API listens, and the key it requires.
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub url: String,
    pub key: SecretString,
}

/// Runtime configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    pub bind_address: SocketAddr,
    pub allowed_origins: Vec<String>,
    pub log_format: LogFormat,
    /// `None` disables media: rooms work, joining returns 503.
    pub livekit: Option<LiveKitConfig>,
    /// `None` disables live translation.
    pub agent: Option<AgentConfig>,
    /// ElevenLabs key for voice cloning; `None` disables recording your voice.
    pub elevenlabs_api_key: Option<SecretString>,
    /// Built SPA (`apps/web/build`) served for non-API paths; `None` serves only the API.
    pub web_dir: Option<PathBuf>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let bind_address = env::var("DICERI_BIND_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1:8090".to_owned())
            .parse()
            .context("DICERI_BIND_ADDRESS must be a socket address")?;
        let allowed_origins = env::var("DICERI_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5180".to_owned())
            .split(',')
            .map(str::trim)
            .filter(|origin| !origin.is_empty())
            .map(str::to_owned)
            .collect();
        let log_format = match env::var("DICERI_LOG_FORMAT").as_deref() {
            Ok("json") => LogFormat::Json,
            _ => LogFormat::Pretty,
        };
        let livekit = livekit_from_env()?;
        let agent = agent_from_env();
        Ok(Self {
            bind_address,
            allowed_origins,
            log_format,
            livekit,
            agent,
            elevenlabs_api_key: env::var("DICERI_ELEVENLABS_API_KEY")
                .ok()
                .map(SecretString::from),
            web_dir: env::var_os("DICERI_WEB_DIR").map(PathBuf::from),
        })
    }

    /// Configuration for tests: no network binding, localhost origin.
    #[must_use]
    pub fn for_tests() -> Self {
        Self {
            bind_address: SocketAddr::from(([127, 0, 0, 1], 0)),
            allowed_origins: vec!["http://localhost:5180".to_owned()],
            log_format: LogFormat::Pretty,
            livekit: None,
            agent: None,
            elevenlabs_api_key: None,
            web_dir: None,
        }
    }
}

fn livekit_from_env() -> anyhow::Result<Option<LiveKitConfig>> {
    let url = env::var("DICERI_LIVEKIT_URL").ok();
    let api_key = env::var("DICERI_LIVEKIT_API_KEY").ok();
    let api_secret = env::var("DICERI_LIVEKIT_API_SECRET").ok();
    match (url, api_key, api_secret) {
        (Some(url), Some(api_key), Some(secret)) => Ok(Some(LiveKitConfig {
            url,
            api_key,
            api_secret: SecretString::from(secret),
        })),
        (None, None, None) => Ok(None),
        _ => anyhow::bail!(
            "set all of DICERI_LIVEKIT_URL, DICERI_LIVEKIT_API_KEY and DICERI_LIVEKIT_API_SECRET"
        ),
    }
}

fn agent_from_env() -> Option<AgentConfig> {
    let url = env::var("DICERI_AGENT_URL").ok()?;
    let key = env::var("DICERI_AGENT_KEY").ok()?;
    Some(AgentConfig {
        url,
        key: SecretString::from(key),
    })
}
