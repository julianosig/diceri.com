use axum::extract::FromRequestParts;
use axum::http::header::{COOKIE, SET_COOKIE};
use axum::http::request::Parts;
use axum::http::{HeaderMap, HeaderValue};
use diceri_domain::{AccessKey, Username};

use crate::{ApiError, AppState};

/// `HttpOnly` cookie holding the account key; scripts on the page never see it.
const COOKIE_NAME: &str = "diceri_account";
/// About a year: accounts have no password, so the cookie is the login.
const MAX_AGE_SECONDS: u32 = 31_536_000;

/// `Set-Cookie` that signs the browser in (`Some`) or out (`None`).
pub(crate) fn session_cookie(
    key: Option<&AccessKey>,
    secure: bool,
) -> [(axum::http::HeaderName, HeaderValue); 1] {
    let (value, max_age) = key.map_or(("", 0), |key| (key.expose(), MAX_AGE_SECONDS));
    let secure = if secure { "; Secure" } else { "" };
    let cookie = format!(
        "{COOKIE_NAME}={value}; Path=/api; HttpOnly; SameSite=Lax; Max-Age={max_age}{secure}"
    );
    // Keys are base62, so the cookie is always a valid header value.
    let header = HeaderValue::from_str(&cookie).unwrap_or(HeaderValue::from_static(""));
    [(SET_COOKIE, header)]
}

fn key_from_cookies(headers: &HeaderMap) -> Option<AccessKey> {
    headers
        .get_all(COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|cookies| cookies.split(';'))
        .find_map(|pair| pair.trim().strip_prefix(COOKIE_NAME)?.strip_prefix('='))
        .and_then(|raw| raw.parse().ok())
}

/// The signed-in account; requests without a valid session cookie get 401.
#[derive(Debug)]
pub(crate) struct CurrentAccount(pub Username);

impl FromRequestParts<AppState> for CurrentAccount {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, ApiError> {
        let key = key_from_cookies(&parts.headers).ok_or(ApiError::Unauthorized)?;
        Ok(Self(state.accounts.authenticate(&key).await?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_key_among_other_cookies() {
        let key = AccessKey::from_random_bytes([7; 16]);
        let mut headers = HeaderMap::new();
        let cookie = format!("theme=dark; {COOKIE_NAME}={}; other=1", key.expose());
        headers.insert(COOKIE, cookie.parse().unwrap());
        assert_eq!(key_from_cookies(&headers), Some(key));
        assert_eq!(key_from_cookies(&HeaderMap::new()), None);
    }

    #[test]
    fn signing_out_expires_the_cookie() {
        let [(_, value)] = session_cookie(None, true);
        let value = value.to_str().unwrap();
        assert!(value.contains("Max-Age=0"));
        assert!(value.contains("HttpOnly"));
        assert!(value.ends_with("; Secure"));
    }
}
