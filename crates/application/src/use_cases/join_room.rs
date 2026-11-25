use std::sync::Arc;

use diceri_domain::RoomId;

use crate::ports::{
    MediaAccessError, MediaAccessIssuer, MediaGrant, MediaRole, RoomIdGenerator, RoomRepository,
    RoomRepositoryError,
};

#[derive(Debug, thiserror::Error)]
pub enum JoinRoomError {
    #[error("room not found")]
    NotFound,
    #[error(transparent)]
    Repository(#[from] RoomRepositoryError),
    #[error(transparent)]
    MediaAccess(#[from] MediaAccessError),
}

/// A participant's ticket into a room's media session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinTicket {
    pub identity: String,
    pub grant: MediaGrant,
}

/// Grants media access to an existing room under a fresh anonymous identity.
#[derive(Clone)]
pub struct JoinRoomService {
    rooms: Arc<dyn RoomRepository>,
    ids: Arc<dyn RoomIdGenerator>,
    media: Arc<dyn MediaAccessIssuer>,
}

impl std::fmt::Debug for JoinRoomService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("JoinRoomService")
            .finish_non_exhaustive()
    }
}

impl JoinRoomService {
    #[must_use]
    pub fn new(
        rooms: Arc<dyn RoomRepository>,
        ids: Arc<dyn RoomIdGenerator>,
        media: Arc<dyn MediaAccessIssuer>,
    ) -> Self {
        Self { rooms, ids, media }
    }

    pub async fn join(&self, room: &RoomId, role: MediaRole) -> Result<JoinTicket, JoinRoomError> {
        if self.rooms.find(room).await?.is_none() {
            return Err(JoinRoomError::NotFound);
        }
        let prefix = match role {
            MediaRole::Speaker => "p",
            MediaRole::Monitor => "monitor",
        };
        let identity = format!("{prefix}-{}", self.ids.generate());
        let grant = self.media.issue(room, &identity, role)?;
        Ok(JoinTicket { identity, grant })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;
    use diceri_domain::Room;

    use super::*;

    struct OneRoom(RoomId);

    #[async_trait]
    impl RoomRepository for OneRoom {
        async fn insert(&self, _: Room) -> Result<(), RoomRepositoryError> {
            Ok(())
        }
        async fn find(&self, id: &RoomId) -> Result<Option<Room>, RoomRepositoryError> {
            Ok((id == &self.0).then(|| Room::new(id.clone())))
        }
    }

    struct FixedIds;

    impl RoomIdGenerator for FixedIds {
        fn generate(&self) -> RoomId {
            RoomId::from_random_bytes([3; 16])
        }
    }

    #[derive(Default)]
    struct RecordingIssuer(Mutex<Vec<(String, MediaRole)>>);

    impl MediaAccessIssuer for RecordingIssuer {
        fn issue(
            &self,
            _: &RoomId,
            identity: &str,
            role: MediaRole,
        ) -> Result<MediaGrant, MediaAccessError> {
            self.0.lock().unwrap().push((identity.to_owned(), role));
            Ok(MediaGrant {
                server_url: "wss://media".to_owned(),
                token: "jwt".to_owned(),
            })
        }
    }

    fn room() -> RoomId {
        RoomId::from_random_bytes([1; 16])
    }

    fn service(issuer: Arc<RecordingIssuer>) -> JoinRoomService {
        JoinRoomService::new(Arc::new(OneRoom(room())), Arc::new(FixedIds), issuer)
    }

    #[tokio::test]
    async fn issues_grant_for_existing_room() {
        let issuer = Arc::new(RecordingIssuer::default());
        let ticket = service(issuer.clone())
            .join(&room(), MediaRole::Speaker)
            .await
            .unwrap();
        assert!(ticket.identity.starts_with("p-"));
        assert_eq!(issuer.0.lock().unwrap()[0].1, MediaRole::Speaker);
    }

    #[tokio::test]
    async fn monitor_identity_is_distinguishable() {
        let ticket = service(Arc::default())
            .join(&room(), MediaRole::Monitor)
            .await
            .unwrap();
        assert!(ticket.identity.starts_with("monitor-"));
    }

    #[tokio::test]
    async fn unknown_room_is_rejected_without_issuing() {
        let issuer = Arc::new(RecordingIssuer::default());
        let unknown = RoomId::from_random_bytes([9; 16]);
        let result = service(issuer.clone())
            .join(&unknown, MediaRole::Speaker)
            .await;
        assert!(matches!(result, Err(JoinRoomError::NotFound)));
        assert!(issuer.0.lock().unwrap().is_empty());
    }
}
