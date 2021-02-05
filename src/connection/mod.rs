use std::sync::Arc;

use bytes::{Buf, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        Mutex,
    },
};

use crate::{config::BUFFER_SIZE, constants::SPLITTER};

#[derive(Debug)]
pub struct Connection {
    pub stream: BufWriter<TcpStream>,
    pub buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(BUFFER_SIZE),
        }
    }

    pub async fn read_to_end(&mut self) -> crate::Result<Option<Vec<u8>>> {
        loop {
            if let Some(index) = self.buffer.iter().position(|i| i == &SPLITTER) {
                let res = hex::decode(&self.buffer[..index])?.to_vec();
                self.buffer.advance(index);
                return Ok(Some(res));
            }
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("faild read".into());
                }
            }
        }
    }
    pub async fn read(&mut self, sender: UnboundedSender<Vec<u8>>) -> crate::Result<()> {
        loop {
            if 0 != self.stream.read_buf(&mut self.buffer).await? || !self.buffer.is_empty() {
                let len = self.buffer.len();
                let len = if len > BUFFER_SIZE { BUFFER_SIZE } else { len };
                if let Some(index) = self.buffer[..len].iter().position(|i| i == &SPLITTER) {
                    let res = hex::decode(&self.buffer[..index])?.to_vec();
                    self.buffer.advance(index);
                    sender.send(res).unwrap();
                    break;
                } else {
                    let res = hex::decode(&self.buffer[..len])?.to_vec();
                    self.buffer.advance(len);
                    sender.send(res).unwrap();
                }
            }
        }
        Ok(())
    }

    pub async fn write_all(&mut self, src: &[u8]) -> crate::Result<()> {
        self.stream.write(hex::encode(src).as_bytes()).await?;
        self.stream.write(&[SPLITTER]).await?;
        self.stream.flush().await?;
        Ok(())
    }

    pub async fn write(&mut self, mut receiver: UnboundedReceiver<Vec<u8>>) -> crate::Result<()> {
        while let Some(src) = receiver.recv().await {
            self.stream.write(hex::encode(src).as_bytes()).await?;
        }
        self.stream.write(&[SPLITTER]).await?;
        self.stream.flush().await?;
        Ok(())
    }
}

pub async fn pipe(conn0: Connection, conn1: Connection) -> crate::Result<()> {
    let conn0 = Arc::new(Mutex::new(conn0));
    let conn1 = Arc::new(Mutex::new(conn1));
    let (sender0, receiver0) = unbounded_channel::<Vec<u8>>();
    let (sender1, receiver1) = unbounded_channel::<Vec<u8>>();
    let conn0_cloned = Arc::clone(&conn0);
    let conn1_cloned = Arc::clone(&conn1);
    let jh = tokio::spawn(async move {
        let conn0 = conn0_cloned;
        let conn1 = conn1_cloned;
        conn1.lock().await.write(receiver0).await.unwrap();
        conn0.lock().await.write(receiver1).await.unwrap();
    });
    conn0.lock().await.read(sender0).await?;
    conn1.lock().await.read(sender1).await?;
    jh.await?;
    Ok(())
}
