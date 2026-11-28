//! The media server's signed reports (ADR 0018): only signed ones are acted on, and the
//! endpoint is not limited per address (every report comes from the media server).
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    reason = "tests fail loudly"
)]

mod support;

use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use diceri_api::{AppState, Config, MediaEvents, build_router};
use diceri_application::ports::{
    CallMembers, LiveRoom, MediaAccessError, MediaEvent, MediaEventError, MediaEventVerifier,
    MediaPresence, MediaRoomCloser,
};
use diceri_application::use_cases::CallEventsService;
use diceri_domain::RoomId;
use tower::ServiceExt as _;

/// Accepts reports signed "good" (the real signature check has its own tests in infra).
struct Signed;

impl MediaEventVerifier for Signed {
    fn verify(&self, _: &str, authorization: &str) -> Result<Option<MediaEvent>, MediaEventError> {
        if authorization != "good" {
            return Err(MediaEventError::Unsigned);
        }
        Ok(Some(MediaEvent::Finished {
            room: RoomId::from_parts(9, [9; 10]),
        }))
    }
}

struct Nobody;

#[async_trait]
impl MediaPresence for Nobody {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError> {
        Ok(Vec::new())
    }

    async fn open_rooms(&self) -> Result<Vec<RoomId>, MediaAccessError> {
        Ok(Vec::new())
    }

    async fn identities_in(&self, _: &RoomId) -> Result<Vec<String>, MediaAccessError> {
        Ok(Vec::new())
    }

    async fn call_members(&self, _: &RoomId) -> Result<CallMembers, MediaAccessError> {
        Ok(CallMembers::default())
    }
}

#[async_trait]
impl MediaRoomCloser for Nobody {
    async fn close(&self, _: &RoomId) -> Result<(), MediaAccessError> {
        Ok(())
    }

    async fn remove(&self, _: &RoomId, _: &str) -> Result<(), MediaAccessError> {
        Ok(())
    }
}

fn app(with_media: bool, write_limit_per_minute: u32) -> axum::Router {
    let mut state = AppState::in_memory(None);
    if with_media {
        let events =
            CallEventsService::new(state.rooms.repository(), Arc::new(Nobody), Arc::new(Nobody));
        state.media_events = Some(MediaEvents::new(Arc::new(Signed), events));
    }
    let config = Config {
        write_limit_per_minute,
        ..Config::for_tests()
    };
    build_router(state, &config)
}

async fn report(app: &axum::Router, authorization: Option<&str>) -> StatusCode {
    let mut request =
        Request::post("/api/v1/livekit/webhook").header("content-type", "application/webhook+json");
    if let Some(authorization) = authorization {
        request = request.header("authorization", authorization);
    }
    let request = request.body(Body::from("{}")).unwrap();
    app.clone().oneshot(request).await.unwrap().status()
}

#[tokio::test]
async fn only_signed_reports_are_accepted() {
    let app = app(true, 0);
    assert_eq!(report(&app, None).await, StatusCode::UNAUTHORIZED);
    assert_eq!(report(&app, Some("forged")).await, StatusCode::UNAUTHORIZED);
    assert_eq!(report(&app, Some("good")).await, StatusCode::OK);
}

#[tokio::test]
async fn reports_are_not_limited_per_address() {
    let app = app(true, 1);
    for _ in 0..3 {
        assert_eq!(report(&app, Some("good")).await, StatusCode::OK);
    }
}

#[tokio::test]
async fn without_a_media_server_there_is_nothing_to_report() {
    let app = app(false, 0);
    assert_eq!(
        report(&app, Some("good")).await,
        StatusCode::SERVICE_UNAVAILABLE
    );
}
