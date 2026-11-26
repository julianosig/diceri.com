//! Diceri HTTP control plane.
//!
//! Routes are grouped by feature under `features/`. This crate wires adapters
//! from `diceri-infra` into use cases from `diceri-application`.

pub mod backup;
mod config;
mod error;
mod features;
mod rate_limit;
mod router;
mod shutdown;
mod state;
pub mod telemetry;

pub use config::{AgentConfig, Config, LiveKitConfig, LogFormat};
pub use error::ApiError;
pub use router::build_router;
pub use shutdown::shutdown_signal;
pub use state::AppState;
