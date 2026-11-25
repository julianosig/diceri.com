use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use diceri_application::use_cases::RoomError;
use serde::Serialize;

/// Errors returned by HTTP handlers, rendered as RFC 9457 problem details.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("resource not found")]
    NotFound,
    #[error("invalid request: {0}")]
    Validation(String),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ProblemDetails {
    title: &'static str,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, title, detail) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found", None),
            Self::Validation(detail) => (StatusCode::BAD_REQUEST, "Bad Request", Some(detail)),
            Self::Internal(error) => {
                tracing::error!(error = ?error, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                    None,
                )
            }
        };
        let body = ProblemDetails {
            title,
            status: status.as_u16(),
            detail,
        };
        let mut response = (status, Json(body)).into_response();
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/problem+json"),
        );
        response
    }
}

impl From<RoomError> for ApiError {
    fn from(error: RoomError) -> Self {
        match error {
            RoomError::NotFound => Self::NotFound,
            RoomError::Repository(source) => Self::Internal(source.into()),
        }
    }
}
