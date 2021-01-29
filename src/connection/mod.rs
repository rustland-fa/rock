use bytes::{Buf, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

use crate::constants::SPLITTER;
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
            last_index: 0,
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

    fn parse_frame_bytes(&mut self) -> crate::Result<Option<Vec<u8>>> {
        let len_buffer = self.buffer[..].len();
        if len_buffer < self.last_index + 2 {
            return Ok(None);
        }
        for i in self.last_index..len_buffer - 1 {
            if self.buffer[i] == b'\r' && self.buffer[i + 1] == b'\n' {
                let res = Some(hex::decode(&self.buffer[..i])?.to_vec());
                self.buffer.advance(len_buffer);
                self.last_index = 0;
                return Ok(res);
            }
        }
        self.last_index = len_buffer - 1;
        Ok(None)
    }

    pub async fn write(&mut self, src: &[u8]) -> crate::Result<()> {
        self.stream.write(hex::encode(src).as_bytes()).await?;
        self.stream.write(SPLITTER).await?;
        self.stream.flush().await?;
        Ok(())
    }
}
