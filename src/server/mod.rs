use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
    time::{sleep, Instant},
};

use crate::constants::DEL_ROOMS_DURATION_SECS;
use crate::{connection::Connection, utils::is_expire};

pub struct Server {
    port: u32,
    banner: String,
    password: String,
    rooms: Arc<Mutex<HashMap<String, Room>>>,
}

pub struct Room {
    first: Connection,
    second: Connection,
    opened: Instant,
    full: bool,
}

impl Server {
    fn new(port: u32, banner: &str, password: &str) -> Self {
        Self {
            port,
            banner: banner.to_string(),
            password: password.to_string(),
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn run(port: u32, banner: &str, password: &str) -> crate::Result<()> {
        let server = Self::new(port, banner, password);
        server.start().await
    }

    async fn start(&self) -> crate::Result<()> {
        //Self::delete_rooms(self.rooms.clone()).await;
        self.listen().await?;
        Ok(())
    }

    async fn delete_rooms(rooms: Arc<Mutex<HashMap<String, Room>>>) {
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(DEL_ROOMS_DURATION_SECS)).await;
                let rooms_cloned = rooms.clone();
                let mut rooms_cloned = rooms_cloned.lock().await;
                rooms_cloned.retain(|_key, room| !is_expire(room.opened));
            }
        });
    }

    pub async fn listen(&self) -> crate::Result<()> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        loop {
            let (socket, socket_addr) = listener.accept().await.unwrap();
            let rooms = self.rooms.clone();
            println!("socket addr => {}", socket_addr);
            tokio::spawn(async move { Self::handler(rooms, socket).await });
        }
    }

    async fn handler(_rooms: Arc<Mutex<HashMap<String, Room>>>, mut socket: TcpStream) {
        println!("handler start");
        let mut buf = String::new();
        socket.read_to_string(&mut buf).await.unwrap();
        println!("read buf {}", buf);
        socket.write(b"thanks").await.unwrap();
    }
}

pub fn pipe() {}
