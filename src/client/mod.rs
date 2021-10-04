use std::{
    convert::{TryFrom, TryInto},
    time::Duration,
};

use crate::message::{Message, PeerType, StartReq, StartTrans};
use futures::{SinkExt, StreamExt};
use log::{error, info};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    time::sleep,
};
use tokio_tungstenite::{connect_async, tungstenite};

#[derive(Debug)]
pub struct Client {
    pub server_addr: String,
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
        server_addr: String,
        username: String,
        password: String,
        room_name: String,
        path_file: String,
        repeat: Option<usize>,
        peer_type: PeerType,
    ) -> Self {
        let repeat = repeat.unwrap_or_default();
        let file_size = match peer_type {
            PeerType::Sender => std::fs::metadata(&path_file).unwrap().len() as usize,
            _ => 0,
        };
        Self {
            server_addr,
            file_size,
            username,
            password,
            path_file,
            room_name,
            repeat,
            peer_type,
        }
    }

    pub async fn connect(mut self) -> crate::Result {
        // TODO auth
        let (socket, _) = connect_async(&self.server_addr).await.unwrap();
        let (mut sink, mut stream) = socket.split();
        let req = Message::StartReq(StartReq {
            peer_type: self.peer_type,
            room_name: self.room_name.clone(),
        });
        sink.send(req.try_into().unwrap()).await.unwrap();
        let mut repeat = self.repeat;
        while let Some(Ok(msg)) = stream.next().await {
            let msg = Message::try_from(&msg).unwrap();
            match msg {
                Message::StartAck(resp) => {
                    info!("receive start ack request message : {}", resp);
                    if self.peer_type == PeerType::Sender {
                        let msg = start_trans_req(&self.path_file, self.file_size).unwrap();
                        sink.send(msg).await.unwrap();
                    }
                }
                Message::StartNack(err) => {
                    error!("failed start request error message : {}", err);
                    return Err(crate::error::Error::ResponseError("".to_string()));
                }
                Message::StartTrans(resp) if self.peer_type == PeerType::Receiver => {
                    self.file_size = resp.total_size;
                    self.path_file = resp.name_file;
                    sink.send(Message::TransAck("ok".to_string()).try_into().unwrap())
                        .await
                        .unwrap();
                }
                Message::TransAck(resp) if self.peer_type == PeerType::Sender => {
                    info!("trans ack receive message : {}", resp);
                    break;
                }
                Message::TransNack(req) if self.peer_type == PeerType::Sender => {
                    info!("trans nack receive error message : {}", req);
                    if repeat == 0 {
                        return Err(crate::error::Error::ResponseError("".to_string()));
                    }
                    sleep(Self::DELAY).await;
                    let msg = start_trans_req(&self.path_file, self.file_size).unwrap();
                    sink.send(msg).await.unwrap();
                    repeat -= 1;
                }
                _ => {
                    error!("invalid request");
                    return Err(crate::error::Error::ResponseError("".to_string()));
                }
            }
        }

        match self.peer_type {
            PeerType::Sender => {
                let mut file = File::open(&self.path_file).await.unwrap();
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).await.unwrap();
                sink.send(tungstenite::Message::Binary(buf)).await.unwrap();
                sink.send(Message::Fin.try_into().unwrap()).await.unwrap();
                sink.close().await.unwrap();
            }
            PeerType::Receiver => {
                let mut buffer = File::create(&self.path_file).await.unwrap();
                while let Some(Ok(msg)) = stream.next().await {
                    if msg.is_binary() {
                        let data = msg.into_data();
                        info!("receiver data len : {}", data.len());
                        buffer.write(&data).await.unwrap();
                    } else if msg.is_text() {
                        let msg = Message::try_from(&msg).unwrap();
                        match msg {
                            Message::Fin => {
                                info!("success download");
                                sink.close().await.unwrap();
                                return Err(crate::error::Error::ResponseError("".to_string()));
                            }
                            _ => {
                                error!("invalid request");
                                return Err(crate::error::Error::ResponseError("".to_string()));
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn start_trans_req(
    path_file: &String,
    file_size: usize,
) -> crate::Result<tungstenite::Message> {
    let trans_req = StartTrans {
        name_file: path_file.clone(),
        total_size: file_size,
    };
    Ok(Message::StartTrans(trans_req).try_into().unwrap())
}
