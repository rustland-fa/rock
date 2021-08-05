use crate::{
    args::Args, codec::MessageCodec, config::ClientConfig, connection::Connection, message::Message,
};
use futures::{AsyncReadExt, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

#[derive(Debug)]
pub struct Client {
    pub config: ClientConfig,
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        Self { config }
    }

    pub async fn connect() {
        let socket = TcpStream::connect(addr).await.unwrap();
        let frame = Framed::new(socket, MessageCodec);
        let (mut sink, mut stream) = frame.split();
        // sink.send(Message::StartReq)
        while let Some(Ok(s)) = stream.next().await {
            match s {
                Message::StartReq(_) => {
                    panic!("")
                }
                Message::StartAck(_) => {
                    // sink.send(Message::StartTrans)
                }
                Message::StartNack(_) => {
                    panic!("")
                }
                Message::StartTrans(_) => {}
                Message::TransAck(_) => {
                    info!("Start Ack ");
                }
                Message::TransNack(_) => {}
                Message::Data(data) => {}
                Message::Fin => {}
            }
        }
    }
}
