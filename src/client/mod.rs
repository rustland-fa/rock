use core::panic;
use std::{
    convert::{TryFrom, TryInto},
    path::PathBuf,
    time::Duration,
};

use crate::{
    config::ClientConfig,
    message::{Message, PeerType, StartReq, StartTrans},
};
use futures::{SinkExt, StreamExt};
use log::info;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite};

#[derive(Debug)]
pub struct Client {
    pub server_addr: url::Url,
    pub username: String,
    pub password: String,
    pub room_name: String,
    pub path_file: String,
    pub repeat: usize,
    pub file_size: usize,
    pub peer_type: PeerType,
}

impl Client {
    pub const DELAY: Duration = Duration::from_secs(3);
    pub fn new(
        server_addr: &String,
        username: String,
        password: String,
        path_file: String,
        room_name: String,
        repeat: usize,
        peer_type: PeerType,
    ) -> Self {
        Self {
            server_addr: url::Url::parse(server_addr).expect("Can't connect to case count URL"),
            file_size: std::fs::metadata(&path_file).unwrap().len() as usize,
            username,
            password,
            path_file,
            room_name,
            repeat,
            peer_type,
        }
    }

    pub async fn connect(&self) {
        let (socket, _) = connect_async(&self.server_addr).await.unwrap();
        let (mut sink, mut stream) = socket.split();
        let req = Message::StartReq(StartReq {
            peer_type: self.peer_type,
            room_name: self.room_name.clone(),
        });
        sink.send(req.try_into().unwrap()).await.unwrap();
        while let Some(Ok(msg)) = stream.next().await {
            let msg = Message::try_from(&msg).unwrap();
            match msg {
                Message::StartAck(req) => {
                    info!("get start ack request {}", req);
                    if self.peer_type == PeerType::Sender {
                        let msg = self.send_start_trans_req();
                        sink.send(msg).await.unwrap();
                    }
                }
                Message::StartNack(_) => {
                    sleep(Self::DELAY).await;
                    let msg = self.send_start_trans_req();
                    sink.send(msg).await.unwrap();
                }
                Message::StartTrans(_) => {
                    if self.peer_type == PeerType::Receiver {
                        // sink.send(Message::TransAck(""))
                    } else {
                        panic!("")
                    }
                }
                Message::TransAck(_) => {
                    info!("Start Ack Send DATA");
                    //  sink.send(Message::Data(""))
                }
                Message::TransNack(_) => {}
                Message::Fin => {
                    // stop
                }
                _ => {}
            }
        }
    }

    pub fn send_start_trans_req(&self) -> tungstenite::Message {
        let trans_req = StartTrans {
            name_file: self.path_file.clone(),
            total_size: self.file_size,
        };
        Message::StartTrans(trans_req).try_into().unwrap()
    }
}
