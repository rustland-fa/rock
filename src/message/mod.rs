use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite;

use crate::utils::{from_json, to_json};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Message {
    StartReq(StartReq),
    StartAck(String),
    StartNack(String),
    StartTrans(StartTrans),
    TransAck(String),
    TransNack(String),
    Fin,
}

impl Message {
    pub fn to_json(&self) -> crate::Result<String> {
        to_json(&self)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartReq {
    pub room_name: String,
    pub peer_type: PeerType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartTrans {
    pub name_file: String,
    pub total_size: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum PeerType {
    Sender,
    Receiver,
}

impl TryFrom<&tungstenite::Message> for Message {
    type Error = String;

    fn try_from(value: &tungstenite::Message) -> Result<Self, Self::Error> {
        if value.is_text() {
            // TODO remove unwrap
            Ok(from_json(value.to_text().unwrap()).unwrap())
        } else {
            Err("message not json ".to_string())
        }
    }
}

impl TryFrom<Message> for tungstenite::Message {
    type Error = String;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        // TODO remove unwrap
        Ok(tungstenite::Message::Text(value.to_json().unwrap()))
    }
}
