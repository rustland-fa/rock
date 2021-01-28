use bytes::{Buf, BytesMut};
use tokio::{
    io::{AsyncReadExt, BufWriter},
    net::TcpStream,
};
#[derive(Debug)]
pub struct Connection {
    pub stream: BufWriter<TcpStream>,
    pub buffer: BytesMut,
    pub last_index: usize,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
            last_index: 1,
        }
    }

    pub async fn read(&mut self) -> crate::Result<Option<Vec<u8>>> {
        loop {
            if let Some(frame) = self.parse_frame_bytes()? {
                return Ok(Some(frame));
            }
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("read data from socket failed".into());
                }
            }
        }
    }

    pub fn parse_frame_bytes(&mut self) -> crate::Result<Option<Vec<u8>>> {
        let mut res = None;
        let mut read_len = 0;
        for (i, b) in self.buffer[self.last_index..].iter().enumerate() {
            read_len = i;
            if self.buffer[i - 1] == b'\r' && *b == b'\n' {
                res = Some(hex::decode(&self.buffer[..i - 2])?.to_vec());
            }
        }
        if res.is_some() {
            self.buffer.advance(read_len);
            self.last_index = 1;
        } else {
            self.last_index = read_len;
        }
        Ok(res)
    }

    pub fn write(&self) {}
}
