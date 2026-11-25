//! Speech-to-text adapters.

mod scribe;
mod scribe_protocol;
mod soniox;
mod soniox_protocol;

pub use scribe::{ScribeConfig, ScribeSpeechToText};
pub use soniox::{SonioxConfig, SonioxSpeechToText};
