//! Per-client limit on writes (POST): scripted abuse of rooms and voice cloning
//! would spend paid provider credits. Reads (polling, presence) stay unlimited.

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::extract::{ConnectInfo, Request, State};
use axum::http::{HeaderMap, HeaderName, Method};
use axum::middleware::Next;
use axum::response::{IntoResponse as _, Response};

use crate::ApiError;

/// Above this many tracked clients, idle ones are forgotten.
const MAX_TRACKED: usize = 10_000;
const IDLE_AFTER: Duration = Duration::from_secs(600);

struct Bucket {
    tokens: f64,
    updated: Instant,
}

/// Token bucket per client IP: `per_minute` writes, bursting up to the same amount.
pub(crate) struct RateLimiter {
    buckets: Mutex<HashMap<IpAddr, Bucket>>,
    capacity: f64,
    refill_per_second: f64,
    /// Header holding the real client IP when behind a proxy (e.g. `cf-connecting-ip`).
    ip_header: Option<HeaderName>,
}

impl RateLimiter {
    #[must_use]
    pub(crate) fn new(per_minute: u32, ip_header: Option<HeaderName>) -> Self {
        let capacity = f64::from(per_minute);
        Self {
            buckets: Mutex::new(HashMap::new()),
            capacity,
            refill_per_second: capacity / 60.0,
            ip_header,
        }
    }

    /// Takes one token for `ip`; `false` when the client is over its limit.
    pub(crate) fn allow(&self, ip: IpAddr, now: Instant) -> bool {
        let Ok(mut buckets) = self.buckets.lock() else {
            return true;
        };
        if buckets.len() > MAX_TRACKED {
            buckets.retain(|_, bucket| now.duration_since(bucket.updated) < IDLE_AFTER);
        }
        let bucket = buckets.entry(ip).or_insert(Bucket {
            tokens: self.capacity,
            updated: now,
        });
        let elapsed = now.duration_since(bucket.updated).as_secs_f64();
        bucket.tokens = elapsed
            .mul_add(self.refill_per_second, bucket.tokens)
            .min(self.capacity);
        bucket.updated = now;
        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    fn client_ip(&self, headers: &HeaderMap, peer: Option<IpAddr>) -> Option<IpAddr> {
        client_ip(headers, peer, self.ip_header.as_ref())
    }
}

/// The client's address: the proxy's header when configured and valid, else the peer.
pub(crate) fn client_ip(
    headers: &HeaderMap,
    peer: Option<IpAddr>,
    ip_header: Option<&HeaderName>,
) -> Option<IpAddr> {
    let from_header =
        ip_header.and_then(|name| headers.get(name)?.to_str().ok()?.trim().parse().ok());
    from_header.or(peer)
}

/// Middleware: rejects POSTs over the limit with `429` problem details.
pub(crate) async fn limit_writes(
    State(limiter): State<Arc<RateLimiter>>,
    request: Request,
    next: Next,
) -> Response {
    if request.method() == Method::POST {
        // Absent in in-process tests; set by `into_make_service_with_connect_info` in `main`.
        let peer = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ConnectInfo(addr)| addr.ip());
        let ip = limiter.client_ip(request.headers(), peer);
        if let Some(ip) = ip
            && !limiter.allow(ip, Instant::now())
        {
            return ApiError::TooManyRequests("slow down and try again in a minute".to_owned())
                .into_response();
        }
    }
    next.run(request).await
}

#[cfg(test)]
mod tests {
    use super::*;

    const IP: IpAddr = IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);

    #[test]
    fn allows_a_burst_then_refills_over_time() {
        let limiter = RateLimiter::new(2, None);
        let start = Instant::now();
        assert!(limiter.allow(IP, start));
        assert!(limiter.allow(IP, start));
        assert!(!limiter.allow(IP, start));
        assert!(limiter.allow(IP, start + Duration::from_secs(30)));
    }

    #[test]
    fn clients_have_separate_buckets() {
        let limiter = RateLimiter::new(1, None);
        let now = Instant::now();
        assert!(limiter.allow(IP, now));
        assert!(limiter.allow("10.0.0.1".parse().unwrap(), now));
    }

    #[test]
    fn the_proxy_header_wins_over_the_peer_address() {
        let limiter = RateLimiter::new(1, Some(HeaderName::from_static("cf-connecting-ip")));
        let mut headers = HeaderMap::new();
        headers.insert("cf-connecting-ip", "203.0.113.9".parse().unwrap());
        assert_eq!(
            limiter.client_ip(&headers, Some(IP)),
            Some("203.0.113.9".parse().unwrap())
        );
        assert_eq!(limiter.client_ip(&HeaderMap::new(), Some(IP)), Some(IP));
    }
}
