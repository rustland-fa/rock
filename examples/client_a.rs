use std::str::from_utf8;

use rock::{connection::Connection, frame::PartFrame};
use tokio::net::TcpStream;
#[tokio::main]
pub async fn main() {
    let socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let mut conn = Connection::new(socket);
    conn.write_all(b"Hello").await.unwrap();
    let data = conn.read_to_end().await.unwrap().unwrap();
    println!(
        "Response Client A Is=> {:?}",
        std::str::from_utf8(&data).unwrap()
    );
    conn.write_all(b"room1").await.unwrap();
    let ok = conn.read_to_end().await.unwrap().unwrap();
    println!("ok => {}", from_utf8(&ok).unwrap());
    conn.read_to_end().await;
    let frame = PartFrame::End("Hello".as_bytes().to_vec());
    conn.write(frame).await.unwrap();
}
