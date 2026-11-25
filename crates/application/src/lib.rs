//! Use cases and ports for Diceri.
//!
//! Ports are traits implemented by adapters in `diceri-infra`. Use cases depend
//! only on ports and domain types, never on concrete providers.

pub mod ports;
pub mod use_cases;
