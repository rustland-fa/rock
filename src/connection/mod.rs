use bytes::BytesMut;
use tokio::{io::BufWriter, net::TcpStream};
#[derive(Debug)]
pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub fn read(&self) {}

    pub fn write(&self) {}
}
