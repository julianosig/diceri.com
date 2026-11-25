//! Text-to-speech adapters.

mod elevenlabs;
mod elevenlabs_voices;
mod pcm;

pub use elevenlabs::{ElevenLabsConfig, ElevenLabsTextToSpeech};
pub use elevenlabs_voices::ElevenLabsVoiceCloner;
