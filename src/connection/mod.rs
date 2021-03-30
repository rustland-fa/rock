use crate::{config::BUFFER_SIZE, constants::SPLITTER, frame::PartFrame};
use bytes::{Buf, BytesMut};
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
    sync::{mpsc::unbounded_channel, Mutex},
};

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
                self.buffer.advance(index + 1);
                return Ok(Some(res));
            }
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err("failed read buffer empty".into())
                }
            }
        }
    }

    pub async fn read(&mut self) -> crate::Result<Option<PartFrame<Vec<u8>>>> {
        if 0 != self.stream.read_buf(&mut self.buffer).await? || !self.buffer.is_empty() {
            let len = self.buffer.len();
            let len = if len > BUFFER_SIZE { BUFFER_SIZE } else { len };
            return if let Some(index) = self.buffer[..len].iter().position(|i| i == &SPLITTER) {
                let res = hex::decode(&self.buffer[..index])?.to_vec();
                self.buffer.advance(index + 1);
                Ok(Some(PartFrame::End(res)))
            } else {
                let res = hex::decode(&self.buffer[..len])?.to_vec();
                println!("res {:?}", res);
                self.buffer.advance(len);
                Ok(Some(PartFrame::Continue(res)))
            }
        }
        Err("failed read buffer empty".into())
    }

    pub async fn write_all(&mut self, src: &[u8]) -> crate::Result<()> {
        self.stream.write(hex::encode(src).as_bytes()).await?;
        self.stream.write(&[SPLITTER]).await?;
        self.stream.flush().await?;
        Ok(())
    }

    pub async fn write(&mut self, src: PartFrame<Vec<u8>>) -> crate::Result<()> {
        match src {
            PartFrame::Continue(src) => {
                self.stream.write(hex::encode(src).as_bytes()).await?;
            }
            PartFrame::End(src) => {
                self.stream.write(hex::encode(src).as_bytes()).await?;
                self.stream.write(&[SPLITTER]).await?;
            }
        }
        self.stream.flush().await?;
        Ok(())
    }
}

pub async fn pipe(conn0: Connection, conn1: Connection) -> crate::Result<()> {
    let conn0 = Arc::new(Mutex::new(conn0));
    let conn1 = Arc::new(Mutex::new(conn1));
    let (sender0, mut receiver0) = unbounded_channel::<PartFrame<Vec<u8>>>();
    let (sender1, mut receiver1) = unbounded_channel::<PartFrame<Vec<u8>>>();
    let conn0_cloned = Arc::clone(&conn0);
    let conn1_cloned = Arc::clone(&conn1);
    let jh = tokio::spawn(async move {
        let conn0 = conn0_cloned;
        let conn1 = conn1_cloned;
        loop {
            let mut guard0 = conn0.lock().await;
            let mut guard1 = conn1.lock().await;
            tokio::select! {
               Ok(Some(data)) = guard0.read() => {
                 sender0.send(data).expect("send data to sender0 failed");
               },
               Ok(Some(data)) = guard1.read() => {
                 sender1.send(data).expect("send data to sender1 failed");
               },
               else => break,
            }
        }
    });

    loop {
        tokio::select! {
            Some(data) = receiver0.recv() => {
                conn1.lock().await.write(data).await?;
            },
            Some(data) = receiver1.recv() => {
                conn0.lock().await.write(data).await?;
            },
            else => break,
        }
    }
    jh.await?;
    Ok(())
}
