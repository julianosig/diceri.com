//! The API serves the built SPA for every non-API path.
#![allow(clippy::unwrap_used, reason = "tests fail loudly")]

use std::path::PathBuf;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use diceri_api::{AppState, Config, build_router};
use http_body_util::BodyExt as _;
use tower::ServiceExt as _;

fn web_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("diceri-web-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("index.html"), "landing").unwrap();
    std::fs::write(dir.join("200.html"), "shell").unwrap();
    dir
}

fn app(dir: PathBuf) -> Router {
    let config = Config {
        web_dir: Some(dir),
        ..Config::for_tests()
    };
    build_router(AppState::in_memory(None), &config)
}

async fn get(app: &Router, uri: &str) -> (StatusCode, String) {
    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

#[tokio::test]
async fn serves_the_landing_page_and_the_shell_for_client_routes() {
    let app = app(web_dir("routes"));
    assert_eq!(get(&app, "/").await, (StatusCode::OK, "landing".to_owned()));
    assert_eq!(
        get(&app, "/r/someroom").await,
        (StatusCode::OK, "shell".to_owned())
    );
}

#[tokio::test]
async fn unknown_api_paths_are_not_found_instead_of_the_shell() {
    let app = app(web_dir("api"));
    let (status, body) = get(&app, "/api/v1/nope").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_ne!(body, "shell");
}

#[tokio::test]
async fn web_files_say_how_long_they_may_be_cached() {
    let app = app(web_dir("cache"));
    let request = Request::builder().uri("/@ana").body(Body::empty()).unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.headers()["cache-control"], "no-cache");
}
