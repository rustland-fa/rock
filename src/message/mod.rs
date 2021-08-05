use crate::{
    compress::{compress, decompress},
    crypt::{decrypt, encrypt},
    utils::{json_bytes_to_data, to_json},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Message {
    StartReq(StartReq),
    StartAck(String),
    StartNack(String),
    StartTrans(String),
    TransAck(String),
    TransNack(String),
    Data(Vec<u8>),
    Fin,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartReq {
    password: String,
    room_name: String,
    pub_key: Option<String>,
}

impl StartReq {
    pub fn client() {}
    pub fn server() {}
}
