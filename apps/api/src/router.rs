use std::time::Duration;

use axum::Router;
use axum::http::{HeaderValue, Method, StatusCode};
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::{AppState, Config, features};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// Builds the full HTTP router with middleware. Used by `main` and tests.
pub fn build_router(state: AppState, config: &Config) -> Router {
    Router::new()
        .merge(features::health::routes())
        .nest("/api/v1", features::api_routes())
        .with_state(state)
        .layer(cors_layer(&config.allowed_origins))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            REQUEST_TIMEOUT,
        ))
        .layer(TraceLayer::new_for_http())
}

fn cors_layer(origins: &[String]) -> CorsLayer {
    let origins: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|origin| origin.parse().ok())
        .collect();
    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE])
}
