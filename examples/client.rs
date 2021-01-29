use rock::connection::Connection;
use tokio::net::TcpStream;
#[tokio::main]
pub async fn main() {
    let socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let mut conn = Connection::new(socket);
    conn.write(b"Hello \r\n Server").await.unwrap();
    let data = conn.read().await.unwrap().unwrap();
    println!("{:?}", std::str::from_utf8(&data).unwrap());
}
