use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use diceri_domain::AccessKey;

use crate::ApiError;

/// Header carrying a host key or a guest's request key.
pub(crate) const ACCESS_KEY_HEADER: &str = "x-diceri-key";

/// Extracts the caller's access key; missing or malformed keys are 401.
#[derive(Debug)]
pub(crate) struct CallerKey(pub AccessKey);

impl<S: Send + Sync> FromRequestParts<S> for CallerKey {
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let key = parts
            .headers
            .get(ACCESS_KEY_HEADER)
            .and_then(|value| value.to_str().ok())
            .and_then(|raw| raw.parse().ok())
            .map(Self)
            .ok_or(ApiError::Unauthorized);
        std::future::ready(key)
    }
}
