use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use diceri_application::ports::SpeechAudio;
use diceri_domain::LanguageCode;
use diceri_media::{Caption, CaptionKind, PipelineOutput};
use livekit::DataPacket;
use livekit::participant::LocalParticipant;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::track_names::{CAPTIONS_TOPIC, translated_track_name};
use crate::translated_track::TranslatedTrack;

/// Delivers pipeline results into the LiveKit room: captions over the data
/// channel and speech on one published track per (speaker, language).
pub(crate) struct LiveKitOutput {
    participant: LocalParticipant,
    tracks: Mutex<HashMap<String, Arc<TranslatedTrack>>>,
}

impl LiveKitOutput {
    pub(crate) fn new(participant: LocalParticipant) -> Self {
        Self {
            participant,
            tracks: Mutex::default(),
        }
    }

    async fn track(&self, name: String) -> anyhow::Result<Arc<TranslatedTrack>> {
        let mut tracks = self.tracks.lock().await;
        if let Some(track) = tracks.get(&name) {
            return Ok(track.clone());
        }
        let track = Arc::new(TranslatedTrack::publish(&self.participant, &name).await?);
        tracing::info!(track = %name, "published translated track");
        tracks.insert(name, track.clone());
        Ok(track)
    }
}

/// Wire format of a caption message (mirrored in the web client).
#[derive(Serialize)]
struct CaptionMessage<'a> {
    speaker: &'a str,
    language: &'a str,
    kind: &'static str,
    phrase: u64,
    text: &'a str,
}

fn kind_name(kind: CaptionKind) -> &'static str {
    match kind {
        CaptionKind::Partial => "partial",
        CaptionKind::Original => "original",
        CaptionKind::Translation => "translation",
    }
}

#[async_trait]
impl PipelineOutput for LiveKitOutput {
    async fn caption(&self, caption: Caption) {
        let message = CaptionMessage {
            speaker: &caption.speaker,
            language: caption.language.as_str(),
            kind: kind_name(caption.kind),
            phrase: caption.phrase,
            text: &caption.text,
        };
        let Ok(payload) = serde_json::to_vec(&message) else {
            return;
        };
        let packet = DataPacket {
            payload,
            topic: Some(CAPTIONS_TOPIC.to_owned()),
            reliable: caption.kind != CaptionKind::Partial,
            ..DataPacket::default()
        };
        if let Err(error) = self.participant.publish_data(packet).await {
            tracing::warn!(%error, "failed to publish caption");
        }
    }

    async fn speak(&self, speaker: &str, language: &LanguageCode, audio: SpeechAudio) {
        match self.track(translated_track_name(speaker, language)).await {
            Ok(track) => track.play(audio).await,
            Err(error) => tracing::warn!(%error, "failed to publish translated track"),
        }
    }
}
