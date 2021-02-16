use tokio::net::TcpStream;

use crate::connection::Connection;

pub async fn new_connection(addr: &str, password: &str, room: &str) -> crate::Result<()> {
    let socket = TcpStream::connect(addr).await?;
    let mut conn = Connection::new(socket);
    // TODO generate STRONG Key
    conn.write_all(password.as_bytes()).await?;
    let ip_and_banner = conn.read_to_end().await?;
    conn.write_all(room.as_bytes()).await?;
    let result = conn.read_to_end().await?;
    Ok(())
}
