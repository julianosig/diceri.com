use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Context as _;
use diceri_application::ports::{MediaAccessIssuer as _, MediaParticipant, MediaRole, VoiceId};
use diceri_domain::{LanguageCode, RoomId};
use diceri_infra::{OUTPUT_LANGUAGE_ATTRIBUTE, VOICE_ATTRIBUTE};
use diceri_media::{Conversation, PipelineOutput};
use livekit::prelude::{
    RemoteParticipant, RemoteTrack, RemoteTrackPublication, Room, RoomEvent, RoomOptions,
};
use livekit::track::TrackSource;
use tokio::sync::{mpsc, watch};

use crate::output::LiveKitOutput;
use crate::providers::Providers;
use crate::speaker_task::{self, Speaker};
use crate::targets::{Member, has_listeners, speech_hints, targets_for};
use crate::track_names::AGENT_IDENTITY;

/// Leave a room after it has had no human participants for this long.
const IDLE_TIMEOUT: Duration = Duration::from_secs(30);
const IDLE_CHECK: Duration = Duration::from_secs(5);

/// Joins one room and translates everyone in it until the room empties.
pub(crate) async fn run(room_id: RoomId, providers: Arc<Providers>) -> anyhow::Result<()> {
    let participant = MediaParticipant {
        identity: AGENT_IDENTITY,
        profile: None,
        role: MediaRole::Translator,
    };
    let grant = providers
        .issuer
        .issue(&room_id, participant)
        .context("agent token")?;
    let (room, events) =
        Room::connect(&providers.livekit_url, &grant.token, RoomOptions::default())
            .await
            .context("connect to room")?;
    tracing::info!(%room_id, "agent joined room");
    serve(&room, events, &providers).await;
    leave(room).await;
    Ok(())
}

/// Reacts to room events until disconnected or idle for [`IDLE_TIMEOUT`].
/// When the host ends the meeting the room is already gone, so closing can fail
/// harmlessly; there is nothing left to clean up on the media side.
async fn leave(room: Room) {
    if let Err(error) = room.close().await {
        tracing::debug!(%error, "room already closed");
    }
}

async fn serve(
    room: &Room,
    mut events: mpsc::UnboundedReceiver<RoomEvent>,
    providers: &Arc<Providers>,
) {
    let output: Arc<dyn PipelineOutput> = Arc::new(LiveKitOutput::new(room.local_participant()));
    // One conversation per call: every speaker is translated knowing what the others said.
    let conversation = Arc::new(Conversation::default());
    let mut speakers = Speakers::default();
    let mut last_human = Instant::now();
    let mut idle_check = tokio::time::interval(IDLE_CHECK);
    loop {
        tokio::select! {
            event = events.recv() => {
                match event {
                    None | Some(RoomEvent::Disconnected { .. }) => break,
                    Some(event) => speakers.handle(event, room, providers, &output, &conversation),
                }
                speakers.refresh_targets(room);
            }
            _ = idle_check.tick() => {
                if has_listeners(members(room).iter().map(Snapshot::member)) {
                    last_human = Instant::now();
                } else if last_human.elapsed() >= IDLE_TIMEOUT {
                    break;
                }
            }
        }
    }
    speakers.delete_all_clones(providers);
}

/// Target-language channels of the speakers currently being translated, and
/// the voice clones to delete when their owners leave.
#[derive(Default)]
struct Speakers {
    targets: HashMap<String, watch::Sender<Vec<LanguageCode>>>,
    clones: HashMap<String, VoiceId>,
}

impl Speakers {
    fn handle(
        &mut self,
        event: RoomEvent,
        room: &Room,
        providers: &Arc<Providers>,
        output: &Arc<dyn PipelineOutput>,
        conversation: &Arc<Conversation>,
    ) {
        if let RoomEvent::ParticipantDisconnected(participant) = &event {
            if let Some(voice) = self.clones.remove(participant.identity().as_str()) {
                delete_clone(providers, voice);
            }
            return;
        }
        let RoomEvent::TrackSubscribed {
            track: RemoteTrack::Audio(track),
            publication,
            participant,
        } = event
        else {
            return;
        };
        let snapshot = members(room);
        let Some(speaker) = speaker(track, &publication, &participant, &snapshot) else {
            return;
        };
        let initial = targets_for(snapshot.iter().map(Snapshot::member), &speaker.identity);
        let (sender, receiver) = watch::channel(initial);
        self.targets.insert(speaker.identity.clone(), sender);
        if let Some(voice) = &speaker.voice {
            self.clones.insert(speaker.identity.clone(), voice.clone());
        }
        tokio::spawn(speaker_task::run(
            speaker,
            providers.clone(),
            output.clone(),
            conversation.clone(),
            receiver,
        ));
    }

    /// Deletes every remaining clone when the session ends.
    fn delete_all_clones(&mut self, providers: &Arc<Providers>) {
        for (_, voice) in self.clones.drain() {
            delete_clone(providers, voice);
        }
    }

    /// Recomputes every speaker's targets after membership or attributes change.
    fn refresh_targets(&mut self, room: &Room) {
        let snapshot = members(room);
        self.targets.retain(|identity, sender| {
            let targets = targets_for(snapshot.iter().map(Snapshot::member), identity);
            sender.send_if_modified(|current| {
                let changed = *current != targets;
                if changed {
                    log_targets(identity, &targets);
                    current.clone_from(&targets);
                }
                changed
            });
            !sender.is_closed()
        });
    }
}

fn log_targets(speaker: &str, targets: &[LanguageCode]) {
    let languages: Vec<&str> = targets.iter().map(LanguageCode::as_str).collect();
    tracing::info!(speaker, ?languages, "translation targets changed");
}

/// Deletes a call's voice clone in the background (ADR 0006). Account clones (ADR 0017)
/// outlive the call: they are deleted only with "Delete my voice".
fn delete_clone(providers: &Arc<Providers>, voice: VoiceId) {
    if providers.keeps_account_voices {
        return;
    }
    let cloner = providers.cloner.clone();
    tokio::spawn(async move {
        match cloner.delete(&voice).await {
            Ok(()) => tracing::info!("voice clone deleted"),
            Err(error) => tracing::warn!(%error, "voice clone could not be deleted"),
        }
    });
}

/// Owned copy of a remote participant's identity and attributes.
struct Snapshot {
    identity: String,
    attributes: HashMap<String, String>,
}

impl Snapshot {
    fn member(&self) -> Member<'_> {
        Member {
            identity: &self.identity,
            attributes: &self.attributes,
        }
    }
}

fn members(room: &Room) -> Vec<Snapshot> {
    room.remote_participants()
        .values()
        .map(|p| Snapshot {
            identity: p.identity().to_string(),
            attributes: p.attributes(),
        })
        .collect()
}

/// A human's microphone track (humans carry an output language); anything else is ignored.
fn speaker(
    track: livekit::track::RemoteAudioTrack,
    publication: &RemoteTrackPublication,
    participant: &RemoteParticipant,
    room: &[Snapshot],
) -> Option<Speaker> {
    if publication.source() != TrackSource::Microphone {
        return None;
    }
    let attributes = participant.attributes();
    let fallback_language = attributes.get(OUTPUT_LANGUAGE_ATTRIBUTE)?.parse().ok()?;
    let voice = attributes
        .get(VOICE_ATTRIBUTE)
        .filter(|voice| !voice.is_empty())
        .map(|voice| VoiceId(voice.clone()));
    let identity = participant.identity().to_string();
    let hints = speech_hints(room.iter().map(Snapshot::member), &identity);
    Some(Speaker {
        identity,
        hints,
        fallback_language,
        voice,
        track,
    })
}
