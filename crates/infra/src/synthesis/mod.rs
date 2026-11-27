//! Text-to-speech adapters.

mod elevenlabs;
mod elevenlabs_voices;
mod inworld;
mod inworld_voices;
mod pcm;
mod provider;

pub use elevenlabs::{ElevenLabsConfig, ElevenLabsTextToSpeech};
pub use elevenlabs_voices::ElevenLabsVoiceCloner;
pub use inworld::{InworldConfig, InworldTextToSpeech};
pub use inworld_voices::InworldVoiceCloner;
pub use provider::TtsProvider;
