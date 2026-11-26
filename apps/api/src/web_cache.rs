//! `Cache-Control` for the web app served by the API. Cloudflare and browsers otherwise
//! guess (static files cached for hours), so a changed file kept its old URL stale.

use axum::extract::Request;
use axum::http::HeaderValue;
use axum::http::header::CACHE_CONTROL;
use axum::middleware::Next;
use axum::response::Response;

/// Build output with content hashes in its names never changes under the same URL.
const IMMUTABLE: &str = "public, max-age=31536000, immutable";
/// The app shell and client routes: always revalidate, so a deploy shows at once.
const REVALIDATE: &str = "no-cache";
/// Other static files (mascots, logo): short, since their names carry no hash.
const SHORT: &str = "public, max-age=300";

/// The policy for a request path, or `None` for API and health routes.
pub(crate) fn cache_policy(path: &str) -> Option<&'static str> {
    if path.starts_with("/api/") || path == "/healthz" {
        return None;
    }
    if path.starts_with("/_app/immutable/") {
        return Some(IMMUTABLE);
    }
    let file = path.rsplit('/').next().unwrap_or_default();
    let is_html = std::path::Path::new(file)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("html"));
    if file.is_empty() || !file.contains('.') || is_html {
        Some(REVALIDATE)
    } else {
        Some(SHORT)
    }
}

/// Middleware: adds the policy to successful web responses that set none.
pub(crate) async fn cache_web_assets(request: Request, next: Next) -> Response {
    let policy = cache_policy(request.uri().path());
    let mut response = next.run(request).await;
    if let Some(policy) = policy
        && response.status().is_success()
        && !response.headers().contains_key(CACHE_CONTROL)
    {
        response
            .headers_mut()
            .insert(CACHE_CONTROL, HeaderValue::from_static(policy));
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashed_build_output_is_immutable() {
        assert_eq!(
            cache_policy("/_app/immutable/assets/scene-dark.Ab12.webp"),
            Some(IMMUTABLE)
        );
    }

    #[test]
    fn pages_revalidate_and_other_files_cache_briefly() {
        assert_eq!(cache_policy("/"), Some(REVALIDATE));
        assert_eq!(cache_policy("/@ana"), Some(REVALIDATE));
        assert_eq!(cache_policy("/200.html"), Some(REVALIDATE));
        assert_eq!(cache_policy("/mascots/webp/robot.webp"), Some(SHORT));
        assert_eq!(cache_policy("/api/v1/presence"), None);
        assert_eq!(cache_policy("/healthz"), None);
    }
}
