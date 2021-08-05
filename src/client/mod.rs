use crate::{args::Args, codec::MessageCodec, connection::Connection, message::Message};
use futures::{AsyncReadExt, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct Client {
    pub args: Args,
    pub conn: TcpStream,
}

impl Client {

}

pub async fn new_connection(addr: &str, password: &str, room: &str) -> crate::Result<()> {
    let socket = TcpStream::connect(addr).await?;
    let frame = Framed::new(socket, MessageCodec);
    let (mut sink, mut stream) = frame.split();
    while let Some(Ok(s)) = stream.next().await {
        match s {
            Message::Fin => {
                break;
            }
            _ => {

            }
        }
    }
    Ok(())
}
