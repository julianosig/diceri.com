use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::http::{HeaderValue, Method, StatusCode};
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::rate_limit::{RateLimiter, limit_writes};
use crate::{AppState, Config, features};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// Builds the full HTTP router with middleware. Used by `main` and tests.
pub fn build_router(state: AppState, config: &Config) -> Router {
    let router = Router::new()
        .merge(features::health::routes())
        .nest("/api/v1", with_write_limit(features::api_routes(), config));
    let router = match &config.web_dir {
        Some(dir) => router.fallback_service(spa(dir)),
        None => router,
    };
    router
        .with_state(state)
        .layer(cors_layer(&config.allowed_origins))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            REQUEST_TIMEOUT,
        ))
        .layer(TraceLayer::new_for_http())
}

fn with_write_limit(routes: Router<AppState>, config: &Config) -> Router<AppState> {
    if config.write_limit_per_minute == 0 {
        return routes;
    }
    let header = config
        .client_ip_header
        .as_deref()
        .and_then(|name| name.parse().ok());
    let limiter = Arc::new(RateLimiter::new(config.write_limit_per_minute, header));
    routes.layer(axum::middleware::from_fn_with_state(limiter, limit_writes))
}

/// Static files of the SPA build; unknown paths get the client-side router shell.
fn spa(dir: &Path) -> ServeDir<ServeFile> {
    ServeDir::new(dir)
        .precompressed_br()
        .precompressed_gzip()
        .fallback(ServeFile::new(dir.join("200.html")))
}

fn cors_layer(origins: &[String]) -> CorsLayer {
    let origins: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();
    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderName::from_static(features::ACCESS_KEY_HEADER),
        ])
}
