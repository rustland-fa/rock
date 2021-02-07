use std::{thread, time::Duration};

use rock::{
    connection::{pipe, Connection},
    progress::ProgressBar,
    server::Server,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let (socket, _addr) = listener.accept().await.unwrap();
    let conn0 = Connection::new(socket);
    let (socket1, _addr) = listener.accept().await.unwrap();
    let conn1 = Connection::new(socket1);
    pipe(conn0, conn1).await.unwrap();
    println!("rock cli tool")
}
