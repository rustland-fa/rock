use rock::server::Server;

#[tokio::main]
async fn main() {
    let server = Server::new("banner", "password");
    server.run("127.0.0.1:8080").await.unwrap();
    println!("rock cli tool");
}
