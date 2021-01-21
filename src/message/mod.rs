use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    type_message: String,
    content_message: String,
    data: Vec<u8>,
    number: i32,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Message {
    pub fn send(&self) -> crate::Result<()> {
        Ok(())
    }

    pub fn encode(&self) {}

    pub fn decode(&self) {}
}
