use crate::{
    constants::DEL_ROOMS_DURATION_SECS,
    message::{Message, PeerType, StartReq, StartTrans},
    server::room::Room,
};
use futures::{stream::SplitSink, AsyncReadExt, SinkExt, StreamExt};
use log::{error, info};
use std::{
    collections::{hash_map::Entry, HashMap},
    convert::{TryFrom, TryInto},
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio::{
    net::{TcpListener, TcpSocket, TcpStream},
    sync::{
        mpsc::{channel, unbounded_channel, UnboundedSender},
        Mutex,
    },
    time::{sleep, Instant, Interval},
};
use tokio_tungstenite::{accept_async, tungstenite, WebSocketStream};

use self::room::{RoomStatus, Rooms};

pub mod room;

#[derive(Debug, Clone)]
pub struct Server {
    pub addr: String,
    pub username: String,
    pub password: String,
    pub rooms: Rooms,
}

impl Server {
    pub fn new(addr: String, username: String, password: String) -> Self {
        Self {
            addr,
            username,
            password,
            rooms: Rooms::new(),
        }
    }

    pub async fn run(self) -> crate::Result<()> {
        let rooms = self.rooms.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                rooms.delete_expired_rooms().await;
            }
        });
        self.start().await
    }

    async fn start(self) -> crate::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        loop {
            let server = self.clone(); // TODO spown scope
            let (socket, socket_addr) = listener.accept().await.unwrap();
            info!("accept socket addr : {}", socket_addr);
            tokio::spawn(async move {
                if let Err(e) = server.handler(socket_addr, socket).await {
                    eprintln!("{}", e);
                }
            });
        }
    }

    async fn handler(self, socket_addr: SocketAddr, socket: TcpStream) -> crate::Result<()> {
        info!("spawn task socket addr : {}", socket_addr);
        let socket = accept_async(socket).await?;
        let (mut sink, mut stream) = socket.split();
        let (sender, mut receiver) = unbounded_channel::<tungstenite::Message>();
        let mut room_name: Option<String> = None;
        loop {
            tokio::select! {
                Some(Ok(message)) = stream.next() => {
                    self.process_msg(&mut room_name, &socket_addr, message, &sender, &mut sink).await?;
                }
                Some(msg) = receiver.recv()  => {
                  sink.send(msg).await?;
                }
            }
        }
    }
    // TODO return boolean
    pub async fn process_msg(
        &self,
        room_name: &mut Option<String>,
        socket_addr: &SocketAddr,
        message: tungstenite::Message,
        sender: &UnboundedSender<tungstenite::Message>,
        sink: &mut SplitSink<WebSocketStream<TcpStream>, tungstenite::Message>,
    ) -> crate::Result<()> {
        if message.is_text() {
            let msg = Message::try_from(&message)?;
            match msg {
                Message::StartReq(req) => {
                    info!("start request addr : {}", socket_addr);
                    if let Err(e) = update_room(&req, &self.rooms, &socket_addr, &sender).await {
                        error!("failed start request error message : {}", e);
                        sink.send(Message::StartNack("failed start ".to_string()).try_into()?)
                            .await?;
                    } else {
                        *room_name = Some(req.room_name);
                        sink.send(Message::StartAck("sucess start ".to_string()).try_into()?)
                            .await?;
                    }
                }
                Message::StartTrans(req) => {
                    info!("start trans addr : {}", socket_addr);
                    let room_name = room_name.as_ref().ok_or("room name not exist")?;
                    if let Err(e) = send_msg_to_receiver(room_name, message, &self.rooms).await {
                        error!("failed start trans error message : {}", e);
                        sink.send(Message::TransNack("failed trans ".to_string()).try_into()?)
                            .await?;
                    }
                }
                Message::TransAck(req) => {
                    info!("trans ack receive addr : {}", socket_addr);
                    let room_name = room_name.as_ref().ok_or("room name not exist")?;
                    if let Err(e) = send_msg_to_receiver(room_name, message, &self.rooms).await {
                        error!("failed start request error message : {}", e);
                    }
                }
                Message::TransNack(req) => {
                    info!("trans nack receive addr : {}", socket_addr);
                    let room_name = room_name.as_ref().ok_or("room name not exist")?;
                    if let Err(e) = send_msg_to_receiver(room_name, message, &self.rooms).await {
                        error!("failed trans nack request error message : {}", e);
                    }
                }
                Message::Fin => {
                    info!("fin request receive addr : {}", socket_addr);
                    let room_name = room_name.as_ref().ok_or("room name not exist")?;
                    if let Err(e) = send_msg_to_receiver(room_name, message, &self.rooms).await {
                        error!("failed finish request error message : {}", e);
                    }
                }
                _ => {}
            }
        } else if message.is_binary()
            && self.rooms.is_trans_state(room_name.as_ref().unwrap()).await
        {
            info!("bin request receive addr : {}", socket_addr);
            let room_name = room_name.as_ref().ok_or("room name not exist")?;
            send_msg_to_receiver(room_name, message, &self.rooms).await?;
        }
        Ok(())
    }
}

pub async fn send_msg_to_receiver(
    room_name: &String,
    msg: tungstenite::Message,
    rooms: &Rooms,
) -> crate::Result<()> {
    let rooms_cloned = rooms.inner.lock().await;
    let room = rooms_cloned.get(room_name).unwrap();
    if !room.is_full() || room.status == RoomStatus::Trans {
        return Err("room not ready to trans".into());
    }
    room.receiver
        .as_ref()
        .unwrap()
        .1
        .send(msg)
        .map_err(|e| e.into())
}

pub async fn update_room(
    req: &StartReq,
    rooms: &Rooms,
    socket_addr: &SocketAddr,
    sender: &UnboundedSender<tokio_tungstenite::tungstenite::Message>,
) -> crate::Result<()> {
    let mut rooms_cloned = rooms.inner.lock().await;
    match rooms_cloned.entry(req.room_name.clone()) {
        Entry::Occupied(room) => {
            let mut room = room.into_mut();
            if !room.is_full() {
                room.status = RoomStatus::Trans;
                match req.peer_type {
                    PeerType::Sender if room.receiver.is_some() => {
                        room.sender = Some((*socket_addr, sender.clone()));
                    }
                    PeerType::Receiver if room.sender.is_some() => {
                        room.receiver = Some((*socket_addr, sender.clone()));
                    }
                    _ => return Err("update failed".into()),
                }
            } else {
            }
        }
        Entry::Vacant(v) => {
            let room = match req.peer_type {
                PeerType::Sender => Room::new(
                    RoomStatus::Start,
                    None,
                    Some((*socket_addr, sender.clone())),
                ),
                PeerType::Receiver => Room::new(
                    RoomStatus::Start,
                    Some((*socket_addr, sender.clone())),
                    None,
                ),
            };
            v.insert(room);
        }
    }
    Ok(())
}
