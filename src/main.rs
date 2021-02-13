

use rock::{
    connection::{pipe, Connection},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let _a = 34;
    let (socket, _addr) = listener.accept().await.unwrap();
    let conn0 = Connection::new(socket);
    let (socket1, _addr) = listener.accept().await.unwrap();
    let conn1 = Connection::new(socket1);
    pipe(conn0, conn1).await.unwrap();
    println!("rock cli tool")
}
