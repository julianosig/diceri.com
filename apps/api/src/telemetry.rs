//! Tracing subscriber setup.

use tracing_subscriber::EnvFilter;

use crate::LogFormat;

/// Installs the global tracing subscriber. Filter with `RUST_LOG`.
pub fn init(format: &LogFormat) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));
    let builder = tracing_subscriber::fmt().with_env_filter(filter);
    match format {
        LogFormat::Json => builder.json().init(),
        LogFormat::Pretty => builder.init(),
    }
}
