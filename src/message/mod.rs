pub struct Message {
    type_message: String,
    content_message: String,
    data: Vec<u8>,
    number: i32,
}

impl std::fmt::Display for Message {}

impl Message {
    pub fn send(&self) -> crate::Result<String> {}

    pub fn encode(&self) {}

    pub fn decode(&self) {}
}
