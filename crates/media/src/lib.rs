//! Real-time translation pipeline, independent of the media transport.
//!
//! One [`SpeakerPipeline`] runs per speaker: it reads that speaker's transcript
//! events, cuts them into phrases, and for every other language in the room
//! translates the phrase and speaks it through a [`PipelineOutput`].

mod caption;
mod clauses;
mod conversation;
mod output;
mod segmenter;
mod speaker;
mod target_worker;
mod voice_choice;

pub use caption::{Caption, CaptionKind};
pub use conversation::Conversation;
pub use output::PipelineOutput;
pub use speaker::{SpeakerConfig, SpeakerPipeline};
