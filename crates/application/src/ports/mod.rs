//! Ports (driven side) that adapters must implement.

mod account_repository;
mod ids;
mod media_access;
mod password_hasher;
mod presence;
mod random_source;
mod room_repository;
mod speech_to_text;
mod text_to_speech;
mod translation_dispatcher;
mod translator;
mod usage_repository;
mod voice_cloner;

pub use account_repository::{
    AccountRepository, AccountRepositoryError, NewSession, SignupNetwork, StoredAccount,
};
pub use ids::{IdGenerator, IdParts};
pub use media_access::{
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaParticipant, MediaRole, MediaRoomCloser,
};
pub use password_hasher::{Clock, PasswordHash, PasswordHasher, PasswordHasherError};
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
pub use translator::{ContextTurn, TranslationRequest, Translator, TranslatorError};
pub use usage_repository::{UsageRepository, UsageRepositoryError};
pub use voice_cloner::{VoiceCloneError, VoiceCloner, VoiceSample};
