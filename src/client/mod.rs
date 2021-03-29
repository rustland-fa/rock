use tokio::net::TcpStream;

use crate::{args::Args, connection::Connection};

pub struct Client {
    pub args: Args,
}

pub async fn new_connection(addr: &str, password: &str, room: &str) -> crate::Result<()> {
    let socket = TcpStream::connect(addr).await?;
    let mut conn = Connection::new(socket);
    // TODO generate STRONG Key
    conn.write_all(password.as_bytes()).await?;
    let _ip_and_banner = conn.read_to_end().await?;
    conn.write_all(room.as_bytes()).await?;
    let _result = conn.read_to_end().await?;
    Ok(())
}
