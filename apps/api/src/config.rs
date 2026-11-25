use std::env;
use std::net::SocketAddr;

use anyhow::Context as _;

/// Log output format, selected with `DICERI_LOG_FORMAT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Pretty,
    Json,
}

/// Runtime configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    pub bind_address: SocketAddr,
    pub allowed_origins: Vec<String>,
    pub log_format: LogFormat,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let bind_address = env::var("DICERI_BIND_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1:8090".to_owned())
            .parse()
            .context("DICERI_BIND_ADDRESS must be a socket address")?;
        let allowed_origins = env::var("DICERI_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:5173".to_owned())
            .split(',')
            .map(str::trim)
            .filter(|origin| !origin.is_empty())
            .map(str::to_owned)
            .collect();
        let log_format = match env::var("DICERI_LOG_FORMAT").as_deref() {
            Ok("json") => LogFormat::Json,
            _ => LogFormat::Pretty,
        };
        Ok(Self {
            bind_address,
            allowed_origins,
            log_format,
        })
    }

    /// Configuration for tests: no network binding, localhost origin.
    #[must_use]
    pub fn for_tests() -> Self {
        Self {
            bind_address: SocketAddr::from(([127, 0, 0, 1], 0)),
            allowed_origins: vec!["http://localhost:5173".to_owned()],
            log_format: LogFormat::Pretty,
        }
    }
}
