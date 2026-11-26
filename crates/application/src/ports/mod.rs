//! Ports (driven side) that adapters must implement.

mod account_repository;
mod ids;
mod media_access;
mod presence;
mod random_source;
mod room_repository;
mod speech_to_text;
mod text_to_speech;
mod translation_dispatcher;
mod translator;
mod voice_cloner;

pub use account_repository::{AccountRepository, AccountRepositoryError};
pub use ids::{IdGenerator, IdParts};
pub use media_access::{
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, MediaRole, MediaRoomCloser,
};
pub use presence::{LiveRoom, MediaPresence};
pub use random_source::RandomSource;
pub use room_repository::{RoomRepository, RoomRepositoryError};
pub use speech_to_text::{
    STT_SAMPLE_RATE, SpeechStream, SpeechToText, SpeechToTextError, TranscriptEvent,
};
pub use text_to_speech::{
    SpeechAudio, SpeechRequest, TTS_SAMPLE_RATE, TextToSpeech, TextToSpeechError, VoiceId,
};
pub use translation_dispatcher::{DispatchError, TranslationDispatcher};
pub use translator::{TranslationRequest, Translator, TranslatorError};
pub use voice_cloner::{VoiceCloneError, VoiceCloner, VoiceSample};
