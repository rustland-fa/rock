use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc::UnboundedSender, Mutex},
    time::Instant,
};
use tokio_tungstenite::tungstenite;

#[derive(Debug, Clone)]
pub struct Rooms {
    pub inner: Arc<Mutex<HashMap<String, Room>>>,
}

#[derive(Debug, Clone)]
pub struct Room {
    pub status: RoomStatus,
    pub sender: Option<(SocketAddr, UnboundedSender<tungstenite::Message>)>,
    pub receiver: Option<(SocketAddr, UnboundedSender<tungstenite::Message>)>,
    pub expire_time: Instant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoomStatus {
    Start,
    Trans,
}

impl Room {
    const EXPIRE_DURATION: Duration = Duration::from_secs(3 * 3600);

    pub fn new(
        status: RoomStatus,
        receiver: Option<(SocketAddr, UnboundedSender<tungstenite::Message>)>,
        sender: Option<(SocketAddr, UnboundedSender<tungstenite::Message>)>,
    ) -> Self {
        Self {
            status,
            sender,
            receiver,
            expire_time: Instant::now() + Self::EXPIRE_DURATION,
        }
    }

    pub fn is_full(&self) -> bool {
        self.sender.is_some() && self.receiver.is_some()
    }

    pub fn is_expire(&self) -> bool {
        self.expire_time
            .checked_duration_since(Instant::now())
            .is_none()
    }
}

impl Rooms {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn is_trans_state(&self, room_name: &String) -> bool {
        let rooms = self.inner.lock().await;
        if let Some(room) = rooms.get(room_name) {
            return room.is_full() && room.status == RoomStatus::Trans;
        } else {
            return false;
        }
    }

    pub async fn is_full(&self, room_name: &String) -> bool {
        let rooms = self.inner.lock().await;
        if let Some(room) = rooms.get(room_name) {
            return room.is_full();
        } else {
            return false;
        }
    }

    pub async fn delete_expired_rooms(&self) {
        let mut rooms = self.inner.lock().await;
        rooms.retain(|_key, room| !room.is_expire());
    }
}
