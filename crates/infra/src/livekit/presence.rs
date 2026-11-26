use std::collections::HashMap;

use async_trait::async_trait;
use diceri_application::ports::{LiveRoom, MediaAccessError, MediaPresence};
use diceri_domain::RoomId;
use futures_util::future::join_all;
use livekit_api::access_token::VideoGrants;
use secrecy::SecretString;
use serde::Deserialize;
use serde_json::json;

use super::OUTPUT_LANGUAGE_ATTRIBUTE;
use super::twirp::TwirpClient;

/// Rooms inspected per snapshot; bounds the fan-out of `ListParticipants` calls.
const MAX_ROOMS: usize = 50;
/// Identity prefix of people in a call (monitors and the translator are excluded).
const PERSON_PREFIX: &str = "p-";

/// Counts people per room with LiveKit's `ListRooms` and `ListParticipants`.
#[derive(Debug, Clone)]
pub struct LiveKitPresence {
    twirp: TwirpClient,
}

impl LiveKitPresence {
    /// `server_url` is the client URL (`wss://…`).
    #[must_use]
    pub fn new(
        server_url: &str,
        api_key: String,
        api_secret: SecretString,
        http: reqwest::Client,
    ) -> Self {
        Self {
            twirp: TwirpClient::new(server_url, api_key, api_secret, http),
        }
    }

    async fn room_names(&self) -> Result<Vec<String>, MediaAccessError> {
        let grants = VideoGrants {
            room_list: true,
            ..VideoGrants::default()
        };
        let listed: RoomList = self.twirp.call("ListRooms", grants, &json!({})).await?;
        Ok(listed
            .rooms
            .into_iter()
            .take(MAX_ROOMS)
            .map(|room| room.name)
            .collect())
    }

    async fn participants(&self, room: String) -> Result<Vec<Participant>, MediaAccessError> {
        let grants = VideoGrants {
            room_admin: true,
            room: room.clone(),
            ..VideoGrants::default()
        };
        let listed: ParticipantList = self
            .twirp
            .call("ListParticipants", grants, &json!({ "room": room }))
            .await?;
        Ok(listed.participants)
    }
}

#[async_trait]
impl MediaPresence for LiveKitPresence {
    async fn live_rooms(&self) -> Result<Vec<LiveRoom>, MediaAccessError> {
        // Media rooms are named after the room id; anything else is not ours.
        let rooms: Vec<RoomId> = self
            .room_names()
            .await?
            .iter()
            .filter_map(|name| name.parse().ok())
            .collect();
        let lists = join_all(rooms.iter().map(|room| self.participants(room.to_string()))).await;
        // A room closing between the two calls is not an error for the whole snapshot.
        Ok(rooms
            .into_iter()
            .zip(lists)
            .filter_map(|(room, list)| Some(live_room(room, list.ok()?)))
            .collect())
    }
}

#[derive(Debug, Deserialize)]
struct RoomList {
    #[serde(default)]
    rooms: Vec<RoomInfo>,
}

#[derive(Debug, Deserialize)]
struct RoomInfo {
    name: String,
}

#[derive(Debug, Deserialize)]
struct ParticipantList {
    #[serde(default)]
    participants: Vec<Participant>,
}

#[derive(Debug, Deserialize)]
struct Participant {
    identity: String,
    #[serde(default)]
    attributes: HashMap<String, String>,
}

fn live_room(room: RoomId, participants: Vec<Participant>) -> LiveRoom {
    let people: Vec<Participant> = participants
        .into_iter()
        .filter(|p| p.identity.starts_with(PERSON_PREFIX))
        .collect();
    LiveRoom {
        room,
        people: people.len(),
        languages: people
            .iter()
            .filter_map(|p| p.attributes.get(OUTPUT_LANGUAGE_ATTRIBUTE)?.parse().ok())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_only_people_and_reads_their_languages() {
        let listed: ParticipantList = serde_json::from_value(json!({ "participants": [
            { "identity": "p-a", "attributes": { "output_language": "pt-BR" } },
            { "identity": "p-b", "attributes": { "output_language": "nonsense!" } },
            { "identity": "monitor-a", "attributes": { "output_language": "en" } },
            { "identity": "diceri-translator" }
        ] }))
        .unwrap();
        let id = RoomId::from_random_bytes([1; 16]);
        let room = live_room(id.clone(), listed.participants);
        assert_eq!(room.room, id);
        assert_eq!(room.people, 2);
        assert_eq!(room.languages, ["pt-BR".parse().unwrap()]);
    }
}
