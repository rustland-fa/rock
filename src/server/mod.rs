use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
    time::sleep,
};

use crate::connection::{pipe, Connection};
use crate::constants::DEL_ROOMS_DURATION_SECS;

use crate::server::room::Room;
pub mod room;

pub struct Server {
    banner: String,
    password: String,
    rooms: Arc<Mutex<HashMap<String, Room>>>,
}

impl Server {
    pub fn new(banner: &str, password: &str) -> Self {
        Self {
            banner: banner.to_string(),
            password: password.to_string(),
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn run(&self, addr: &str) -> crate::Result<()> {
        Self::delete_rooms(self.rooms.clone()).await;
        self.start(addr).await
    }

    async fn delete_rooms(rooms: Arc<Mutex<HashMap<String, Room>>>) {
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(DEL_ROOMS_DURATION_SECS)).await;
                let rooms_cloned = rooms.clone();
                let mut rooms_cloned = rooms_cloned.lock().await;
                rooms_cloned.retain(|_key, room| !room.is_expire());
            }
        });
    }

    async fn start(&self, addr: &str) -> crate::Result<()> {
        let listener = TcpListener::bind(addr).await?;
        loop {
            let (socket, socket_addr) = listener.accept().await.unwrap();
            let rooms = self.rooms.clone();
            println!("socket addr => {}", socket_addr);
            tokio::spawn(async move { Self::handler(rooms, socket).await });
        }
    }

    async fn handler(
        rooms: Arc<Mutex<HashMap<String, Room>>>,
        socket: TcpStream,
    ) -> crate::Result<()> {
        println!("handler start");
        let mut conn = Connection::new(socket);
        let data = conn.read_to_end().await?;
        if let Some(d) = data {
            let msg = std::str::from_utf8(&d)?;
            println!("msg =>{}", msg);
            // TODO generate strong key
            // TODO fix bug and improve code
            conn.write_all(b"please send key ").await?;
            let key_data = conn.read_to_end().await?.unwrap();
            let key_room = std::str::from_utf8(&key_data)?;
            let mut guard = rooms.lock().await;
            if let Some(room) = guard.get_mut(key_room) {
                if !room.is_full() && !room.is_expire() {
                    conn.write_all(b"ok").await.unwrap();
                    room.set_second_conn(conn);
                } else {
                    let msg = if room.is_full() {
                        "room is full"
                    } else {
                        "room is expire"
                    };
                    conn.write_all(msg.as_bytes()).await?;
                    return Ok(());
                }
                let room = guard.remove(key_room).unwrap();
                pipe(room.first, room.second.unwrap()).await.unwrap();
            } else {
                conn.write_all(b"ok").await.unwrap();
                let room = Room::new(conn);
                guard.insert(key_room.to_string(), room);
                println!("add new room with key {}", key_room);
            }
        }
        Ok(())
    }
}
