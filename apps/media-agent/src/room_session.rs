use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::Context as _;
use diceri_application::ports::{MediaAccessIssuer as _, MediaParticipant, MediaRole};
use diceri_domain::{LanguageCode, RoomId};
use diceri_infra::LANGUAGE_ATTRIBUTE;
use diceri_media::PipelineOutput;
use livekit::prelude::{
    RemoteParticipant, RemoteTrack, RemoteTrackPublication, Room, RoomEvent, RoomOptions,
};
use livekit::track::TrackSource;
use tokio::sync::{mpsc, watch};

use crate::output::LiveKitOutput;
use crate::providers::Providers;
use crate::speaker_task::{self, Speaker};
use crate::targets::{Member, has_listeners, targets_for};
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
    room.close().await.context("leave room")
}

/// Reacts to room events until disconnected or idle for [`IDLE_TIMEOUT`].
async fn serve(
    room: &Room,
    mut events: mpsc::UnboundedReceiver<RoomEvent>,
    providers: &Arc<Providers>,
) {
    let output: Arc<dyn PipelineOutput> = Arc::new(LiveKitOutput::new(room.local_participant()));
    let mut speakers = Speakers::default();
    let mut last_human = Instant::now();
    let mut idle_check = tokio::time::interval(IDLE_CHECK);
    loop {
        tokio::select! {
            event = events.recv() => {
                match event {
                    None | Some(RoomEvent::Disconnected { .. }) => return,
                    Some(event) => speakers.handle(event, room, providers, &output),
                }
                speakers.refresh_targets(room);
            }
            _ = idle_check.tick() => {
                if has_listeners(members(room).iter().map(Snapshot::member)) {
                    last_human = Instant::now();
                } else if last_human.elapsed() >= IDLE_TIMEOUT {
                    return;
                }
            }
        }
    }
}

/// Target-language channels of the speakers currently being translated.
#[derive(Default)]
struct Speakers {
    targets: HashMap<String, watch::Sender<Vec<LanguageCode>>>,
}

impl Speakers {
    fn handle(
        &mut self,
        event: RoomEvent,
        room: &Room,
        providers: &Arc<Providers>,
        output: &Arc<dyn PipelineOutput>,
    ) {
        let RoomEvent::TrackSubscribed {
            track: RemoteTrack::Audio(track),
            publication,
            participant,
        } = event
        else {
            return;
        };
        let Some(speaker) = speaker(track, &publication, &participant) else {
            return;
        };
        let snapshot = members(room);
        let initial = targets_for(snapshot.iter().map(Snapshot::member), &speaker.identity);
        let (sender, receiver) = watch::channel(initial);
        self.targets.insert(speaker.identity.clone(), sender);
        tokio::spawn(speaker_task::run(
            speaker,
            providers.clone(),
            output.clone(),
            receiver,
        ));
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

/// A human's microphone track with a declared language; anything else is ignored.
fn speaker(
    track: livekit::track::RemoteAudioTrack,
    publication: &RemoteTrackPublication,
    participant: &RemoteParticipant,
) -> Option<Speaker> {
    if publication.source() != TrackSource::Microphone {
        return None;
    }
    let language = participant
        .attributes()
        .get(LANGUAGE_ATTRIBUTE)?
        .parse()
        .ok()?;
    Some(Speaker {
        identity: participant.identity().to_string(),
        language,
        track,
    })
}
