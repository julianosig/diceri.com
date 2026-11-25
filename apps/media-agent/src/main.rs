//! Diceri media agent: joins rooms on request and translates every speaker.

mod config;
mod control;
mod output;
mod providers;
mod room_session;
mod sessions;
mod speaker_task;
mod targets;
mod track_names;
mod translated_track;
mod voices;

use std::sync::Arc;

use anyhow::Context as _;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // A missing .env is fine: production injects real environment variables.
    let _ = dotenvy::dotenv();
    diceri_infra::install_tls_provider();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,livekit=warn,libwebrtc=error")),
        )
        .init();
    let config = config::AgentConfig::from_env().context("invalid agent configuration")?;
    let providers = Arc::new(providers::Providers::from_config(&config));
    let router = control::router(
        sessions::Sessions::new(providers),
        config.control_key.clone(),
    );
    let listener = tokio::net::TcpListener::bind(config.bind_address)
        .await
        .with_context(|| format!("failed to bind {}", config.bind_address))?;
    tracing::info!(address = %config.bind_address, "media agent listening");
    axum::serve(listener, router)
        .await
        .context("control server failed")
}
