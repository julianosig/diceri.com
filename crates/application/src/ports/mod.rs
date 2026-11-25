//! Ports (driven side) that adapters must implement.

mod media_access;
mod random_source;
mod room_repository;
mod speech_to_text;
mod text_to_speech;
mod translator;

pub use media_access::{
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, MediaRole,
};
pub use random_source::RandomSource;
pub use room_repository::{RoomRepository, RoomRepositoryError};
pub use speech_to_text::{
    STT_SAMPLE_RATE, SpeechStream, SpeechToText, SpeechToTextError, TranscriptEvent,
};
pub use text_to_speech::{
    SpeechAudio, SpeechRequest, TTS_SAMPLE_RATE, TextToSpeech, TextToSpeechError, VoiceId,
};
pub use translator::{TranslationRequest, Translator, TranslatorError};
