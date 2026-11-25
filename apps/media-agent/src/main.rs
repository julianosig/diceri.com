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
use diceri_application::use_cases::CLONE_NAME_PREFIX;
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
    let providers = Arc::new(providers::Providers::from_config(&config)?);
    // No call is active yet, so any existing clone was orphaned by a crash or restart.
    match providers.cloner.delete_all_named(CLONE_NAME_PREFIX).await {
        Ok(count) => tracing::info!(count, "deleted orphaned voice clones"),
        Err(error) => tracing::warn!(%error, "could not delete orphaned voice clones"),
    }
    tracing::info!(stt = ?config.stt_provider, "speech-to-text provider");
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
