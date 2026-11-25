//! Diceri HTTP control plane.
//!
//! Routes are grouped by feature under `features/`. This crate wires adapters
//! from `diceri-infra` into use cases from `diceri-application`.

mod config;
mod error;
mod features;
mod router;
mod shutdown;
mod state;
pub mod telemetry;

pub use config::{Config, LiveKitConfig, LogFormat};
pub use error::ApiError;
pub use router::build_router;
pub use shutdown::shutdown_signal;
pub use state::AppState;
