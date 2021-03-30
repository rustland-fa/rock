use crate::connection::Connection;
use std::time::Duration;
use tokio::time::Instant;

pub struct Room {
    pub first: Connection,
    pub second: Option<Connection>,
    pub expire_time: Instant,
}

impl Room {
    const EXPIRE_DURATION: Duration = Duration::from_secs(3 * 3600);

    pub fn new(conn: Connection) -> Self {
        Self {
            first: conn,
            second: None,
            expire_time: Instant::now() + Self::EXPIRE_DURATION,
        }
    }

    pub fn set_second_conn(&mut self, conn: Connection) {
        self.second = Some(conn);
    }

    pub fn is_full(&self) -> bool {
        self.second.is_some()
    }

    pub fn is_expire(&self) -> bool {
        self.expire_time
            .checked_duration_since(Instant::now())
            .is_none()
    }
}
