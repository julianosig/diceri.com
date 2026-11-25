//! Text-to-speech adapters.

mod elevenlabs;
mod pcm;

pub use elevenlabs::{ElevenLabsConfig, ElevenLabsTextToSpeech};
