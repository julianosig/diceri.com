use std::collections::HashSet;
use std::sync::{Arc, Mutex, PoisonError};

use diceri_domain::RoomId;

use crate::providers::Providers;
use crate::room_session;

/// Rooms the agent is currently in; starting a running room is a no-op.
pub(crate) struct Sessions {
    running: Mutex<HashSet<RoomId>>,
    providers: Arc<Providers>,
}

impl Sessions {
    pub(crate) fn new(providers: Arc<Providers>) -> Arc<Self> {
        Arc::new(Self {
            running: Mutex::default(),
            providers,
        })
    }

    /// Returns true when a new session was started.
    pub(crate) fn ensure(self: &Arc<Self>, room: RoomId) -> bool {
        let inserted = self
            .running
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(room.clone());
        if inserted {
            let sessions = self.clone();
            tokio::spawn(async move {
                if let Err(error) =
                    room_session::run(room.clone(), sessions.providers.clone()).await
                {
                    tracing::error!(error = ?error, %room, "room session failed");
                }
                sessions
                    .running
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .remove(&room);
            });
        }
        inserted
    }
}
