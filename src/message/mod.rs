use crate::compress::{compress, decompress};
use crate::crypt::{decrypt, encrypt};
use crate::utils::{json_bytes_to_data, to_json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    type_msg: String,
    content: String,
    data: Vec<u8>,
    number: i32,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = to_json(self).unwrap(); //TODO remove unwrap
        writeln!(f, "{}", json)
    }
}

impl Message {
    pub fn new(type_msg: &str, content: &str, data: Vec<u8>, number: i32) -> Self {
        Self {
            type_msg: type_msg.to_string(),
            content: content.to_string(),
            data,
            number,
        }
    }

    pub fn encode(&self, key: &[u8]) -> crate::Result<Vec<u8>> {
        let json_msg = to_json(&self)?;
        let compress_msg = compress(json_msg.as_bytes())?;
        let cipher_msg = encrypt(key, &compress_msg)?;
        Ok(cipher_msg)
    }

    pub fn decode(key: &[u8], cipher_msg: &[u8]) -> crate::Result<Self> {
        let decrypt_msg = decrypt(key, cipher_msg)?;
        let uncompressed_data = decompress(&decrypt_msg)?;
        let msg: Message = json_bytes_to_data(&uncompressed_data)?;
        Ok(msg)
    }
}

pub fn send() -> crate::Result<()> {
    Ok(())
}
