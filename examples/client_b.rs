use rock::connection::Connection;
use tokio::net::TcpStream;
#[tokio::main]
pub async fn main() {
    let socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let mut conn = Connection::new(socket);
    conn.write_all(b"Hello-To-A").await.unwrap();
    let data = conn.read_to_end().await.unwrap().unwrap();
    println!(
        "Response Client B is => {:?}",
        std::str::from_utf8(&data).unwrap()
    );
}
