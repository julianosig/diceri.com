use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::extract::Request;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse as _, Response};
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;

use crate::log_path::redacted_path;
use crate::rate_limit::{RateLimiter, limit_writes};
use crate::web_cache::cache_web_assets;
use crate::{AppState, Config, features};

/// Most requests answer in milliseconds; this bounds a stuck one.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
/// Recording a voice first transcribes it (is it speech?) and then clones it.
const VOICE_TIMEOUT: Duration = Duration::from_secs(45);

/// Builds the full HTTP router with middleware. Used by `main` and tests.
pub fn build_router(state: AppState, config: &Config) -> Router {
    let router = Router::new()
        .merge(features::health::routes())
        .nest("/api/v1", with_write_limit(features::api_routes(), config));
    let router = match &config.web_dir {
        Some(dir) => router
            .fallback_service(spa(dir))
            .layer(axum::middleware::from_fn(cache_web_assets)),
        None => router,
    };
    router
        .with_state(state)
        .layer(cors_layer(&config.allowed_origins))
        .layer(axum::middleware::from_fn(limit_time))
        .layer(TraceLayer::new_for_http().make_span_with(request_span))
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

/// Answers 408 when a request runs past its time limit (dropping it cancels the work).
async fn limit_time(request: Request, next: Next) -> Response {
    // Recording, reusing or deleting a voice waits for the cloning provider.
    let path = request.uri().path();
    let records = request.method() == Method::POST
        && (path.ends_with("/voices") || path.ends_with("/voices/kept"));
    let deletes = request.method() == Method::DELETE && path.ends_with("/accounts/me/voice");
    let limit = if records || deletes {
        VOICE_TIMEOUT
    } else {
        REQUEST_TIMEOUT
    };
    tokio::time::timeout(limit, next.run(request))
        .await
        .unwrap_or_else(|_| StatusCode::REQUEST_TIMEOUT.into_response())
}

/// One span per request with the method and a path safe to log (no codes, usernames or
/// query string): the default span would record the full URI.
fn request_span(request: &Request) -> tracing::Span {
    tracing::info_span!(
        "request",
        method = %request.method(),
        path = %redacted_path(request.uri().path()),
    )
}
