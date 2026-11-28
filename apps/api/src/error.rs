use axum::Json;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use diceri_application::ports::VoiceCloneError;
use diceri_application::use_cases::{
    AccountError, AdmissionError, LimitError, RoomError, VoiceError,
};
use diceri_domain::RoomRuleError;
use serde::Serialize;

/// Errors returned by HTTP handlers, rendered as RFC 9457 problem details.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("resource not found")]
    NotFound,
    #[error("invalid request: {0}")]
    Validation(String),
    #[error("missing or malformed access key")]
    Unauthorized,
    #[error("access key does not allow this action")]
    Forbidden,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("gone: {0}")]
    Gone(String),
    #[error("too many requests: {0}")]
    TooManyRequests(String),
    #[error("service unavailable: {0}")]
    Unavailable(&'static str),
    /// A plan or capacity limit: 429 with a machine-readable `limit` (daily, monthly, busy).
    #[error("limit reached: {detail}")]
    Limit {
        limit: &'static str,
        detail: &'static str,
    },
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ProblemDetails {
    title: &'static str,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    /// Which limit was reached, so the web app can explain it in the visitor's language.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<&'static str>,
}

impl ApiError {
    fn parts(self) -> (StatusCode, &'static str, Option<String>) {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not Found", None),
            Self::Validation(detail) => (StatusCode::BAD_REQUEST, "Bad Request", Some(detail)),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized", None),
            Self::Forbidden => (StatusCode::FORBIDDEN, "Forbidden", None),
            Self::Conflict(detail) => (StatusCode::CONFLICT, "Conflict", Some(detail)),
            Self::Gone(detail) => (StatusCode::GONE, "Gone", Some(detail)),
            Self::TooManyRequests(detail) => (
                StatusCode::TOO_MANY_REQUESTS,
                "Too Many Requests",
                Some(detail),
            ),
            Self::Unavailable(detail) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service Unavailable",
                Some(detail.to_owned()),
            ),
            Self::Limit { detail, .. } => (
                StatusCode::TOO_MANY_REQUESTS,
                "Too Many Requests",
                Some(detail.to_owned()),
            ),
            Self::Internal(error) => {
                tracing::error!(error = ?error, "internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                    None,
                )
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let limit = match &self {
            Self::Limit { limit, .. } => Some(*limit),
            _ => None,
        };
        let (status, title, detail) = self.parts();
        let body = ProblemDetails {
            title,
            status: status.as_u16(),
            detail,
            limit,
        };
        let mut response = (status, Json(body)).into_response();
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/problem+json"),
        );
        response
    }
}

impl From<LimitError> for ApiError {
    fn from(error: LimitError) -> Self {
        match error {
            LimitError::DailyMinutes => Self::Limit {
                limit: "daily",
                detail: "today's call minutes are used up; they renew at midnight UTC",
            },
            LimitError::PlanMonthly => Self::Limit {
                limit: "quota",
                detail: "this month's call minutes on your plan are used up",
            },
            LimitError::MonthlyCapacity => Self::Limit {
                limit: "monthly",
                detail: "this month's free capacity is used up",
            },
            LimitError::Busy => Self::Limit {
                limit: "busy",
                detail: "too many calls are live right now; try again in a few minutes",
            },
            LimitError::Usage(source) => Self::Internal(source.into()),
            LimitError::Accounts(source) => Self::Internal(source.into()),
            LimitError::Rooms(source) => Self::Internal(source.into()),
            LimitError::RoomRule(source) => Self::Internal(source.into()),
        }
    }
}

impl From<AccountError> for ApiError {
    fn from(error: AccountError) -> Self {
        match error {
            AccountError::Taken => Self::Conflict(error.to_string()),
            AccountError::BetaFull => Self::Limit {
                limit: "beta",
                detail: "the public beta has no free spots right now",
            },
            AccountError::NetworkUsed => Self::Limit {
                limit: "network",
                detail: "this network already claimed as many room usernames as allowed",
            },
            AccountError::Unauthorized => Self::Unauthorized,
            AccountError::Repository(source) => Self::Internal(source.into()),
            AccountError::Hasher(source) => Self::Internal(source.into()),
            AccountError::WeakPassword(source) => Self::Validation(source.to_string()),
        }
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

impl From<AdmissionError> for ApiError {
    fn from(error: AdmissionError) -> Self {
        match error {
            AdmissionError::NotFound | AdmissionError::Rule(RoomRuleError::UnknownRequest) => {
                Self::NotFound
            }
            AdmissionError::Forbidden => Self::Forbidden,
            AdmissionError::Rule(rule @ RoomRuleError::Ended) => Self::Gone(rule.to_string()),
            AdmissionError::Rule(RoomRuleError::Full) => Self::Limit {
                limit: "full",
                detail: "the call is full; try again when someone leaves",
            },
            AdmissionError::Rule(rule @ RoomRuleError::AlreadyDecided) => {
                Self::Conflict(rule.to_string())
            }
            AdmissionError::Rule(
                rule @ (RoomRuleError::TooManyPending | RoomRuleError::TooManyVoices),
            ) => Self::TooManyRequests(rule.to_string()),
            AdmissionError::MediaUnavailable => Self::Unavailable("media server not configured"),
            AdmissionError::Repository(source) => Self::Internal(source.into()),
            AdmissionError::MediaAccess(source) => Self::Internal(source.into()),
        }
    }
}

impl From<VoiceError> for ApiError {
    fn from(error: VoiceError) -> Self {
        match error {
            VoiceError::NotFound | VoiceError::NoSavedVoice => Self::NotFound,
            VoiceError::InvalidSample => Self::Validation(error.to_string()),
            VoiceError::NoSpeech => Self::Validation(
                "we could not hear you clearly; read the sentence out loud and try again"
                    .to_owned(),
            ),
            VoiceError::SpeechCheck(source) => {
                tracing::warn!(error = %source, "voice sample could not be checked");
                Self::Unavailable("voice check unavailable, try again shortly")
            }
            VoiceError::Rule(rule) => Self::TooManyRequests(rule.to_string()),
            VoiceError::Clone(source) => clone_failed(source),
            VoiceError::Repository(source) => Self::Internal(source.into()),
            VoiceError::Samples(source) => Self::Internal(source.into()),
            VoiceError::Interrupted => Self::Internal(anyhow::anyhow!("voice operation stopped")),
        }
    }
}

/// The cloning provider refused or failed: a refused sample is the caller's to redo.
fn clone_failed(error: VoiceCloneError) -> ApiError {
    match error {
        VoiceCloneError::Rejected(detail) => {
            tracing::warn!(%detail, "voice sample rejected by provider");
            ApiError::Validation(
                "the recording could not be used; try again in a quieter place".to_owned(),
            )
        }
        source @ VoiceCloneError::Request(_) => ApiError::Internal(source.into()),
    }
}
