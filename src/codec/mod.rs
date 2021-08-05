use crate::{message::Message, utils::to_json};
use bytes::{Buf, BufMut, BytesMut};
use std::io;
use tokio_util::codec::{Decoder, Encoder, LinesCodec};

pub struct MessageCodec;

impl Encoder<Message> for MessageCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let data = to_json(&item).unwrap();
        let data = data.as_bytes();
        dst.reserve(data.len() + 1);
        dst.put(data);
        dst.put_u8(b'\n');
        Ok(())
    }
}

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}
