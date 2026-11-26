//! Binary entry point. Keep this file thin: load config, init telemetry, serve.

use anyhow::Context as _;
use diceri_api::{AppState, Config, build_router, shutdown_signal, telemetry};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // A missing .env is fine: production injects real environment variables.
    let _ = dotenvy::dotenv();
    diceri_infra::install_tls_provider();
    let config = Config::from_env().context("invalid configuration")?;
    telemetry::init(&config.log_format);

    let listener = TcpListener::bind(config.bind_address)
        .await
        .with_context(|| format!("failed to bind {}", config.bind_address))?;
    tracing::info!(address = %config.bind_address, "diceri-api listening");
    if config.livekit.is_none() {
        tracing::warn!("LiveKit not configured: joining rooms will return 503");
    }

    let state = AppState::from_config(&config).await?;
    let router = build_router(state, &config);
    // Peer addresses feed the per-client write limit.
    let service = router.into_make_service_with_connect_info::<std::net::SocketAddr>();
    axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")?;

    tracing::info!("diceri-api stopped");
    Ok(())
}
